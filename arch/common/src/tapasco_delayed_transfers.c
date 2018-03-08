//
// Copyright (C) 2018 Jens Korinth, TU Darmstadt
//
// This file is part of Tapasco (TAPASCO).
//
// Tapasco is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Tapasco is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Tapasco.  If not, see <http://www.gnu.org/licenses/>.
//
/** @file	tapasco_delayed_transfers.c
 *  @brief	Functions for delayed memory transfers.
 *  @author	J. Korinth, TU Darmstadt (jk@esa.cs.tu-darmstadt.de)
 **/
#include <tapasco.h>
#include <tapasco_delayed_transfers.h>
#include <tapasco_logging.h>
#include <tapasco_context.h>
#include <tapasco_device.h>
#include <platform.h>

tapasco_res_t tapasco_transfer_to(tapasco_dev_ctx_t *dev_ctx,
		tapasco_job_id_t const j_id,
		tapasco_transfer_t *t,
		tapasco_slot_id_t s_id)
{
	LOG(LALL_TRANSFERS, "job %lu: executing transfer to with length %zd bytes",
			(unsigned long)j_id, (unsigned long)t->len);
	tapasco_res_t res = tapasco_device_alloc(dev_ctx, &t->handle, t->len,
			t->flags, s_id);
	if (res != TAPASCO_SUCCESS) {
		ERR("job %lu: memory allocation failed!", (unsigned long)j_id);
		return res;
	}
	res = tapasco_device_copy_to(dev_ctx, t->data, t->handle, t->len,
			t->flags, s_id);
	if (res != TAPASCO_SUCCESS) {
		ERR("job %lu: transfer failed - %zd bytes -> 0x%08x with flags %lu",
				(unsigned long)j_id, t->len,
				(unsigned long)t->handle,
				(unsigned long)t->flags);
	}
	return res;
}

tapasco_res_t tapasco_transfer_from(tapasco_dev_ctx_t *dev_ctx,
		tapasco_jobs_t *jobs,
		tapasco_job_id_t const j_id,
		tapasco_transfer_t *t,
		tapasco_slot_id_t s_id)
{
	LOG(LALL_TRANSFERS, "job %lu: executing transfer from with length %zd bytes",
			(unsigned long)j_id, (unsigned long)t->len);
	tapasco_res_t res = tapasco_device_copy_from(dev_ctx, t->handle,
			t->data, t->len, t->flags, s_id);
	if (res != TAPASCO_SUCCESS) {
		ERR("job %lu: transfer failed - %zd bytes <- 0x%08x with flags %lu",
				(unsigned long)j_id, t->len,
				(unsigned long)t->handle,
				(unsigned long)t->flags);
	}
	tapasco_device_free(dev_ctx, t->handle, t->flags, s_id, t->len);
	return res;
}

tapasco_res_t tapasco_write_arg(tapasco_dev_ctx_t *dev_ctx,
		tapasco_jobs_t *jobs,
		tapasco_job_id_t const j_id,
		tapasco_handle_t const h,
		size_t const a)
{
	int const is64 = tapasco_jobs_is_arg_64bit(jobs, j_id, a);
	platform_ctx_t *pctx = tapasco_device_platform(dev_ctx);
	if (is64) {
		uint64_t v = tapasco_jobs_get_arg64(jobs, j_id, a);
		LOG(LALL_TRANSFERS, "job %lu: writing 64b arg #%u = 0x%08lx to 0x%08x",
			(unsigned long)j_id, a, (unsigned long)v, (unsigned)h);
		if (platform_write_ctl(pctx, h, sizeof(v), &v,
				PLATFORM_CTL_FLAGS_NONE) != PLATFORM_SUCCESS)
			return TAPASCO_ERR_PLATFORM_FAILURE;
	} else {
		uint32_t v = tapasco_jobs_get_arg32(jobs, j_id, a);
		LOG(LALL_TRANSFERS, "job %lu: writing 32b arg #%u = 0x%08lx to 0x%08x",
			(unsigned long)j_id, a, (unsigned long)v, (unsigned)h);
		if (platform_write_ctl(pctx, h, sizeof(v), &v,
				PLATFORM_CTL_FLAGS_NONE) != PLATFORM_SUCCESS)
			return TAPASCO_ERR_PLATFORM_FAILURE;
	}
	return TAPASCO_SUCCESS;
}

tapasco_res_t tapasco_read_arg(tapasco_dev_ctx_t *dev_ctx,
		tapasco_jobs_t *jobs,
		tapasco_job_id_t const j_id,
		tapasco_handle_t const h,
		size_t const a)
{
	int const is64 = tapasco_jobs_is_arg_64bit(jobs, j_id, a);
	platform_ctx_t *pctx = tapasco_device_platform(dev_ctx);
	if (is64) {
		uint64_t v = 0;
		if (platform_read_ctl(pctx, h, sizeof(v), &v,
				PLATFORM_CTL_FLAGS_NONE) != PLATFORM_SUCCESS)
			return TAPASCO_ERR_PLATFORM_FAILURE;
		LOG(LALL_TRANSFERS, "job %lu: reading 64b arg #%u = 0x%08lx from 0x%08x",
			(unsigned long)j_id, a, (unsigned long)v, (unsigned)h);
		tapasco_jobs_set_arg(jobs, j_id, a, sizeof(v), &v);
	} else {
		uint32_t v = 0;
		if (platform_read_ctl(pctx, h, sizeof(v), &v,
				PLATFORM_CTL_FLAGS_NONE) != PLATFORM_SUCCESS)
			return TAPASCO_ERR_PLATFORM_FAILURE;
		LOG(LALL_TRANSFERS, "job %lu: reading 32b arg #%u = 0x%08lx from 0x%08x",
			(unsigned long)j_id, a, (unsigned long)v, (unsigned)h);
		tapasco_jobs_set_arg(jobs, j_id, a, sizeof(v), &v);
	}
	return TAPASCO_SUCCESS;
}
