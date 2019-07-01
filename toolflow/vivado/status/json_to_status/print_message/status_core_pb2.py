# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: status_core.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='status_core.proto',
  package='tapasco.status',
  syntax='proto3',
  serialized_pb=_b('\n\x11status_core.proto\x12\x0etapasco.status\"n\n\x02PE\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\n\n\x02id\x18\x02 \x01(\r\x12\x0e\n\x06offset\x18\x03 \x01(\x04\x12\x0c\n\x04size\x18\x04 \x01(\x04\x12\x30\n\x0clocal_memory\x18\x05 \x01(\x0b\x32\x1a.tapasco.status.MemoryArea\"6\n\x08Platform\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0e\n\x06offset\x18\x02 \x01(\x04\x12\x0c\n\x04size\x18\x03 \x01(\x04\",\n\x05\x43lock\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x15\n\rfrequency_mhz\x18\x02 \x01(\r\":\n\x07Version\x12\x10\n\x08software\x18\x01 \x01(\t\x12\x0c\n\x04year\x18\x02 \x01(\r\x12\x0f\n\x07release\x18\x03 \x01(\r\"(\n\nMemoryArea\x12\x0c\n\x04\x62\x61se\x18\x01 \x01(\x04\x12\x0c\n\x04size\x18\x02 \x01(\x04\"\x9b\x02\n\x06Status\x12\x11\n\ttimestamp\x18\x01 \x01(\x04\x12-\n\tarch_base\x18\x02 \x01(\x0b\x32\x1a.tapasco.status.MemoryArea\x12\x31\n\rplatform_base\x18\x03 \x01(\x0b\x32\x1a.tapasco.status.MemoryArea\x12\x1e\n\x02pe\x18\x04 \x03(\x0b\x32\x12.tapasco.status.PE\x12*\n\x08platform\x18\x05 \x03(\x0b\x32\x18.tapasco.status.Platform\x12%\n\x06\x63locks\x18\x06 \x03(\x0b\x32\x15.tapasco.status.Clock\x12)\n\x08versions\x18\x07 \x03(\x0b\x32\x17.tapasco.status.Versionb\x06proto3')
)




_PE = _descriptor.Descriptor(
  name='PE',
  full_name='tapasco.status.PE',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='tapasco.status.PE.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='id', full_name='tapasco.status.PE.id', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='offset', full_name='tapasco.status.PE.offset', index=2,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='size', full_name='tapasco.status.PE.size', index=3,
      number=4, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='local_memory', full_name='tapasco.status.PE.local_memory', index=4,
      number=5, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=37,
  serialized_end=147,
)


_PLATFORM = _descriptor.Descriptor(
  name='Platform',
  full_name='tapasco.status.Platform',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='tapasco.status.Platform.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='offset', full_name='tapasco.status.Platform.offset', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='size', full_name='tapasco.status.Platform.size', index=2,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=149,
  serialized_end=203,
)


_CLOCK = _descriptor.Descriptor(
  name='Clock',
  full_name='tapasco.status.Clock',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='tapasco.status.Clock.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='frequency_mhz', full_name='tapasco.status.Clock.frequency_mhz', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=205,
  serialized_end=249,
)


_VERSION = _descriptor.Descriptor(
  name='Version',
  full_name='tapasco.status.Version',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='software', full_name='tapasco.status.Version.software', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='year', full_name='tapasco.status.Version.year', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='release', full_name='tapasco.status.Version.release', index=2,
      number=3, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=251,
  serialized_end=309,
)


_MEMORYAREA = _descriptor.Descriptor(
  name='MemoryArea',
  full_name='tapasco.status.MemoryArea',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='base', full_name='tapasco.status.MemoryArea.base', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='size', full_name='tapasco.status.MemoryArea.size', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=311,
  serialized_end=351,
)


_STATUS = _descriptor.Descriptor(
  name='Status',
  full_name='tapasco.status.Status',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='timestamp', full_name='tapasco.status.Status.timestamp', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='arch_base', full_name='tapasco.status.Status.arch_base', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='platform_base', full_name='tapasco.status.Status.platform_base', index=2,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='pe', full_name='tapasco.status.Status.pe', index=3,
      number=4, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='platform', full_name='tapasco.status.Status.platform', index=4,
      number=5, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='clocks', full_name='tapasco.status.Status.clocks', index=5,
      number=6, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='versions', full_name='tapasco.status.Status.versions', index=6,
      number=7, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=354,
  serialized_end=637,
)

_PE.fields_by_name['local_memory'].message_type = _MEMORYAREA
_STATUS.fields_by_name['arch_base'].message_type = _MEMORYAREA
_STATUS.fields_by_name['platform_base'].message_type = _MEMORYAREA
_STATUS.fields_by_name['pe'].message_type = _PE
_STATUS.fields_by_name['platform'].message_type = _PLATFORM
_STATUS.fields_by_name['clocks'].message_type = _CLOCK
_STATUS.fields_by_name['versions'].message_type = _VERSION
DESCRIPTOR.message_types_by_name['PE'] = _PE
DESCRIPTOR.message_types_by_name['Platform'] = _PLATFORM
DESCRIPTOR.message_types_by_name['Clock'] = _CLOCK
DESCRIPTOR.message_types_by_name['Version'] = _VERSION
DESCRIPTOR.message_types_by_name['MemoryArea'] = _MEMORYAREA
DESCRIPTOR.message_types_by_name['Status'] = _STATUS
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

PE = _reflection.GeneratedProtocolMessageType('PE', (_message.Message,), dict(
  DESCRIPTOR = _PE,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.PE)
  ))
_sym_db.RegisterMessage(PE)

Platform = _reflection.GeneratedProtocolMessageType('Platform', (_message.Message,), dict(
  DESCRIPTOR = _PLATFORM,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.Platform)
  ))
_sym_db.RegisterMessage(Platform)

Clock = _reflection.GeneratedProtocolMessageType('Clock', (_message.Message,), dict(
  DESCRIPTOR = _CLOCK,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.Clock)
  ))
_sym_db.RegisterMessage(Clock)

Version = _reflection.GeneratedProtocolMessageType('Version', (_message.Message,), dict(
  DESCRIPTOR = _VERSION,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.Version)
  ))
_sym_db.RegisterMessage(Version)

MemoryArea = _reflection.GeneratedProtocolMessageType('MemoryArea', (_message.Message,), dict(
  DESCRIPTOR = _MEMORYAREA,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.MemoryArea)
  ))
_sym_db.RegisterMessage(MemoryArea)

Status = _reflection.GeneratedProtocolMessageType('Status', (_message.Message,), dict(
  DESCRIPTOR = _STATUS,
  __module__ = 'status_core_pb2'
  # @@protoc_insertion_point(class_scope:tapasco.status.Status)
  ))
_sym_db.RegisterMessage(Status)


# @@protoc_insertion_point(module_scope)
