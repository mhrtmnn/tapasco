/*
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo 
 * (see https://github.com/esa-tu-darmstadt/tapasco).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
#include <linux/of.h>
#include <linux/fs.h>
#include <linux/io.h>
#include "tlkm_logging.h"
#include "tlkm_types.h"
#include "tlkm_bus.h"
#include "zynq.h"
#include "zynqmp.h"
#include "zynq_device.h"
#include "zynq_irq.h"
#include "zynq_dmamgmt.h"

static const struct of_device_id zynq_ids[] = {
	{
		.compatible = ZYNQ_NAME,
	},
	{},
};

static const struct of_device_id zynqmp_ids[] = {
	{
		.compatible = ZYNQMP_NAME,
	},
	{},
};

static struct zynq_device _zynq_dev; // there is at most one Zynq

int zynq_device_init(struct tlkm_device *inst, void *data)
{
#ifndef NDEBUG
	if (!inst) {
		ERR("called with NULL device instance");
		return -EACCES;
	}
#endif /* NDEBUG */
	DEVLOG(inst->dev_id, TLKM_LF_DEVICE, "initializing zynq device");
	inst->private_data = &_zynq_dev;
	_zynq_dev.parent = inst;

	DEVLOG(inst->dev_id, TLKM_LF_DEVICE, "zynq successfully initialized");
	return 0;
}

void zynq_device_exit(struct tlkm_device *inst)
{
#ifndef NDEBUG
	if (!inst) {
		ERR("called with NULL device instance");
		return;
	}
#endif /* NDEBUG */
	inst->private_data = NULL;
	DEVLOG(_zynq_dev.parent->dev_id, TLKM_LF_DEVICE, "zynq device exited");
}

int zynq_device_init_subsystems(struct tlkm_device *dev, void *data)
{
	int ret = 0;
	if ((ret = zynq_dmamgmt_init())) {
		DEVERR(dev->dev_id, "could not initialize DMA management: %d",
		       ret);
		goto err_dmamgmt;
	}

	_zynq_dev.requested_irq_num = 0;

	DEVLOG(dev->dev_id, TLKM_LF_DEVICE,
	       "successfully initialized subsystems");
err_dmamgmt:
	return ret;
}

void zynq_device_exit_subsystems(struct tlkm_device *dev)
{
	zynq_dmamgmt_exit(dev);
	DEVLOG(dev->dev_id, TLKM_LF_DEVICE, "exited subsystems");
}

int zynq_device_probe(struct tlkm_class *cls)
{
	struct tlkm_device *inst;
	LOG(TLKM_LF_DEVICE,
	    "searching for Xilinx Zynq-7000 series devices ...");
	if (of_find_matching_node(NULL, zynq_ids)) {
		LOG(TLKM_LF_DEVICE, "found Xilinx Zynq-7000");
		inst = tlkm_bus_new_device(cls, 0, 0, NULL);
		if (!inst)
			return -EFAULT;
	} else {
		LOG(TLKM_LF_DEVICE, "no Xilinx Zynq-7000 series device found");
	}
	return 0;
}

int zynqmp_device_probe(struct tlkm_class *cls)
{
	struct tlkm_device *inst;
	struct device_node *nd;
	LOG(TLKM_LF_DEVICE, "searching for Xilinx Zynq-MP series devices ...");
	if ((nd = of_find_matching_node(NULL, zynqmp_ids))) {
		LOG(TLKM_LF_DEVICE, "found Xilinx Zynq-MP");
		pr_alert("===> of node phandle=%x\n", nd->phandle);
		inst = tlkm_bus_new_device(cls, 0, 0, NULL);
		if (!inst)
			return -EFAULT;
	} else {
		LOG(TLKM_LF_DEVICE, "no Xilinx Zynq-MP series device found");
	}
	return 0;
}
