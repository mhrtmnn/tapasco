/* Automatically generated nanopb header */
/* Generated by nanopb-0.4.0-dev */

#ifndef PB_TAPASCO_STATUS_STATUS_CORE_PB_H_INCLUDED
#define PB_TAPASCO_STATUS_STATUS_CORE_PB_H_INCLUDED
#include <pb.h>

/* @@protoc_insertion_point(includes) */
#if PB_PROTO_HEADER_VERSION != 40
#error Regenerate this file with the current version of nanopb generator.
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* Struct definitions */
typedef struct _tapasco_status_Clock {
    pb_callback_t name;
    uint32_t frequency_mhz;
/* @@protoc_insertion_point(struct:tapasco_status_Clock) */
} tapasco_status_Clock;


typedef struct _tapasco_status_MemoryArea {
    uint64_t base;
    uint64_t size;
/* @@protoc_insertion_point(struct:tapasco_status_MemoryArea) */
} tapasco_status_MemoryArea;


typedef struct _tapasco_status_Platform {
    pb_callback_t name;
    uint64_t offset;
    uint64_t size;
/* @@protoc_insertion_point(struct:tapasco_status_Platform) */
} tapasco_status_Platform;


typedef struct _tapasco_status_Version {
    pb_callback_t software;
    uint32_t year;
    uint32_t release;
/* @@protoc_insertion_point(struct:tapasco_status_Version) */
} tapasco_status_Version;


typedef struct _tapasco_status_PE {
    pb_callback_t name;
    uint32_t id;
    uint64_t offset;
    uint64_t size;
    tapasco_status_MemoryArea local_memory;
/* @@protoc_insertion_point(struct:tapasco_status_PE) */
} tapasco_status_PE;


typedef struct _tapasco_status_Status {
    uint64_t timestamp;
    tapasco_status_MemoryArea arch_base;
    tapasco_status_MemoryArea platform_base;
    pb_callback_t pe;
    pb_callback_t platform;
    pb_callback_t clocks;
    pb_callback_t versions;
/* @@protoc_insertion_point(struct:tapasco_status_Status) */
} tapasco_status_Status;


/* Initializer values for message structs */
#define tapasco_status_PE_init_default           {{{NULL}, NULL}, 0, 0, 0, tapasco_status_MemoryArea_init_default}
#define tapasco_status_Platform_init_default     {{{NULL}, NULL}, 0, 0}
#define tapasco_status_Clock_init_default        {{{NULL}, NULL}, 0}
#define tapasco_status_Version_init_default      {{{NULL}, NULL}, 0, 0}
#define tapasco_status_MemoryArea_init_default   {0, 0}
#define tapasco_status_Status_init_default       {0, tapasco_status_MemoryArea_init_default, tapasco_status_MemoryArea_init_default, {{NULL}, NULL}, {{NULL}, NULL}, {{NULL}, NULL}, {{NULL}, NULL}}
#define tapasco_status_PE_init_zero              {{{NULL}, NULL}, 0, 0, 0, tapasco_status_MemoryArea_init_zero}
#define tapasco_status_Platform_init_zero        {{{NULL}, NULL}, 0, 0}
#define tapasco_status_Clock_init_zero           {{{NULL}, NULL}, 0}
#define tapasco_status_Version_init_zero         {{{NULL}, NULL}, 0, 0}
#define tapasco_status_MemoryArea_init_zero      {0, 0}
#define tapasco_status_Status_init_zero          {0, tapasco_status_MemoryArea_init_zero, tapasco_status_MemoryArea_init_zero, {{NULL}, NULL}, {{NULL}, NULL}, {{NULL}, NULL}, {{NULL}, NULL}}

/* Field tags (for use in manual encoding/decoding) */
#define tapasco_status_Clock_name_tag            1
#define tapasco_status_Clock_frequency_mhz_tag   2
#define tapasco_status_MemoryArea_base_tag       1
#define tapasco_status_MemoryArea_size_tag       2
#define tapasco_status_Platform_name_tag         1
#define tapasco_status_Platform_offset_tag       2
#define tapasco_status_Platform_size_tag         3
#define tapasco_status_Version_software_tag      1
#define tapasco_status_Version_year_tag          2
#define tapasco_status_Version_release_tag       3
#define tapasco_status_PE_name_tag               1
#define tapasco_status_PE_id_tag                 2
#define tapasco_status_PE_offset_tag             3
#define tapasco_status_PE_size_tag               4
#define tapasco_status_PE_local_memory_tag       5
#define tapasco_status_Status_timestamp_tag      1
#define tapasco_status_Status_arch_base_tag      2
#define tapasco_status_Status_platform_base_tag  3
#define tapasco_status_Status_pe_tag             4
#define tapasco_status_Status_platform_tag       5
#define tapasco_status_Status_clocks_tag         6
#define tapasco_status_Status_versions_tag       7

/* Struct field encoding specification for nanopb */
#define tapasco_status_PE_FIELDLIST(X, a) \
X(a, CALLBACK, SINGULAR, STRING, name, 1) \
X(a, STATIC, SINGULAR, UINT32, id, 2) \
X(a, STATIC, SINGULAR, UINT64, offset, 3) \
X(a, STATIC, SINGULAR, UINT64, size, 4) \
X(a, STATIC, SINGULAR, MESSAGE, local_memory, 5)
#define tapasco_status_PE_CALLBACK pb_default_field_callback
#define tapasco_status_PE_DEFAULT NULL
#define tapasco_status_PE_local_memory_MSGTYPE tapasco_status_MemoryArea

#define tapasco_status_Platform_FIELDLIST(X, a) \
X(a, CALLBACK, SINGULAR, STRING, name, 1) \
X(a, STATIC, SINGULAR, UINT64, offset, 2) \
X(a, STATIC, SINGULAR, UINT64, size, 3)
#define tapasco_status_Platform_CALLBACK pb_default_field_callback
#define tapasco_status_Platform_DEFAULT NULL

#define tapasco_status_Clock_FIELDLIST(X, a) \
X(a, CALLBACK, SINGULAR, STRING, name, 1) \
X(a, STATIC, SINGULAR, UINT32, frequency_mhz, 2)
#define tapasco_status_Clock_CALLBACK pb_default_field_callback
#define tapasco_status_Clock_DEFAULT NULL

#define tapasco_status_Version_FIELDLIST(X, a) \
X(a, CALLBACK, SINGULAR, STRING, software, 1) \
X(a, STATIC, SINGULAR, UINT32, year, 2) \
X(a, STATIC, SINGULAR, UINT32, release, 3)
#define tapasco_status_Version_CALLBACK pb_default_field_callback
#define tapasco_status_Version_DEFAULT NULL

#define tapasco_status_MemoryArea_FIELDLIST(X, a) \
X(a, STATIC, SINGULAR, UINT64, base, 1) \
X(a, STATIC, SINGULAR, UINT64, size, 2)
#define tapasco_status_MemoryArea_CALLBACK NULL
#define tapasco_status_MemoryArea_DEFAULT NULL

#define tapasco_status_Status_FIELDLIST(X, a) \
X(a, STATIC, SINGULAR, UINT64, timestamp, 1) \
X(a, STATIC, SINGULAR, MESSAGE, arch_base, 2) \
X(a, STATIC, SINGULAR, MESSAGE, platform_base, 3) \
X(a, CALLBACK, REPEATED, MESSAGE, pe, 4) \
X(a, CALLBACK, REPEATED, MESSAGE, platform, 5) \
X(a, CALLBACK, REPEATED, MESSAGE, clocks, 6) \
X(a, CALLBACK, REPEATED, MESSAGE, versions, 7)
#define tapasco_status_Status_CALLBACK pb_default_field_callback
#define tapasco_status_Status_DEFAULT NULL
#define tapasco_status_Status_arch_base_MSGTYPE tapasco_status_MemoryArea
#define tapasco_status_Status_platform_base_MSGTYPE tapasco_status_MemoryArea
#define tapasco_status_Status_pe_MSGTYPE tapasco_status_PE
#define tapasco_status_Status_platform_MSGTYPE tapasco_status_Platform
#define tapasco_status_Status_clocks_MSGTYPE tapasco_status_Clock
#define tapasco_status_Status_versions_MSGTYPE tapasco_status_Version

extern const pb_msgdesc_t tapasco_status_PE_msg;
extern const pb_msgdesc_t tapasco_status_Platform_msg;
extern const pb_msgdesc_t tapasco_status_Clock_msg;
extern const pb_msgdesc_t tapasco_status_Version_msg;
extern const pb_msgdesc_t tapasco_status_MemoryArea_msg;
extern const pb_msgdesc_t tapasco_status_Status_msg;

/* Defines for backwards compatibility with code written before nanopb-0.4.0 */
#define tapasco_status_PE_fields &tapasco_status_PE_msg
#define tapasco_status_Platform_fields &tapasco_status_Platform_msg
#define tapasco_status_Clock_fields &tapasco_status_Clock_msg
#define tapasco_status_Version_fields &tapasco_status_Version_msg
#define tapasco_status_MemoryArea_fields &tapasco_status_MemoryArea_msg
#define tapasco_status_Status_fields &tapasco_status_Status_msg

/* Maximum encoded size of messages (where known) */
/* tapasco_status_PE_size depends on runtime parameters */
/* tapasco_status_Platform_size depends on runtime parameters */
/* tapasco_status_Clock_size depends on runtime parameters */
/* tapasco_status_Version_size depends on runtime parameters */
#define tapasco_status_MemoryArea_size           22
/* tapasco_status_Status_size depends on runtime parameters */

#ifdef __cplusplus
} /* extern "C" */
#endif
/* @@protoc_insertion_point(eof) */

#endif
