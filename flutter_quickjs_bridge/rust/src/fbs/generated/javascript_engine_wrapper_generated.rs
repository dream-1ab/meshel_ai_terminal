// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate serde;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod javascript_engine_dart_wrapper {

  use core::mem;
  use core::cmp::Ordering;

  extern crate serde;
  use self::serde::ser::{Serialize, Serializer, SerializeStruct};

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum DartFunctionOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct DartFunction<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DartFunction<'a> {
  type Inner = DartFunction<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> DartFunction<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_POINTER: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    DartFunction { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args DartFunctionArgs<'args>
  ) -> flatbuffers::WIPOffset<DartFunction<'bldr>> {
    let mut builder = DartFunctionBuilder::new(_fbb);
    builder.add_pointer(args.pointer);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(DartFunction::VT_NAME, None)}
  }
  #[inline]
  pub fn pointer(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(DartFunction::VT_POINTER, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for DartFunction<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<u64>("pointer", Self::VT_POINTER, false)?
     .finish();
    Ok(())
  }
}
pub struct DartFunctionArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub pointer: u64,
}
impl<'a> Default for DartFunctionArgs<'a> {
  #[inline]
  fn default() -> Self {
    DartFunctionArgs {
      name: None,
      pointer: 0,
    }
  }
}

impl Serialize for DartFunction<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("DartFunction", 2)?;
      if let Some(f) = self.name() {
        s.serialize_field("name", &f)?;
      } else {
        s.skip_field("name")?;
      }
      s.serialize_field("pointer", &self.pointer())?;
    s.end()
  }
}

pub struct DartFunctionBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> DartFunctionBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(DartFunction::VT_NAME, name);
  }
  #[inline]
  pub fn add_pointer(&mut self, pointer: u64) {
    self.fbb_.push_slot::<u64>(DartFunction::VT_POINTER, pointer, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> DartFunctionBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    DartFunctionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DartFunction<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for DartFunction<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("DartFunction");
      ds.field("name", &self.name());
      ds.field("pointer", &self.pointer());
      ds.finish()
  }
}
pub enum DartModuleOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct DartModule<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DartModule<'a> {
  type Inner = DartModule<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> DartModule<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_FUNCTIONS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    DartModule { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args DartModuleArgs<'args>
  ) -> flatbuffers::WIPOffset<DartModule<'bldr>> {
    let mut builder = DartModuleBuilder::new(_fbb);
    if let Some(x) = args.functions { builder.add_functions(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(DartModule::VT_NAME, None)}
  }
  #[inline]
  pub fn functions(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DartFunction<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DartFunction>>>>(DartModule::VT_FUNCTIONS, None)}
  }
}

impl flatbuffers::Verifiable for DartModule<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<DartFunction>>>>("functions", Self::VT_FUNCTIONS, false)?
     .finish();
    Ok(())
  }
}
pub struct DartModuleArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub functions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DartFunction<'a>>>>>,
}
impl<'a> Default for DartModuleArgs<'a> {
  #[inline]
  fn default() -> Self {
    DartModuleArgs {
      name: None,
      functions: None,
    }
  }
}

impl Serialize for DartModule<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("DartModule", 2)?;
      if let Some(f) = self.name() {
        s.serialize_field("name", &f)?;
      } else {
        s.skip_field("name")?;
      }
      if let Some(f) = self.functions() {
        s.serialize_field("functions", &f)?;
      } else {
        s.skip_field("functions")?;
      }
    s.end()
  }
}

pub struct DartModuleBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> DartModuleBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(DartModule::VT_NAME, name);
  }
  #[inline]
  pub fn add_functions(&mut self, functions: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<DartFunction<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(DartModule::VT_FUNCTIONS, functions);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> DartModuleBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    DartModuleBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DartModule<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for DartModule<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("DartModule");
      ds.field("name", &self.name());
      ds.field("functions", &self.functions());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `DartModule`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_dart_module_unchecked`.
pub fn root_as_dart_module(buf: &[u8]) -> Result<DartModule, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<DartModule>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `DartModule` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_dart_module_unchecked`.
pub fn size_prefixed_root_as_dart_module(buf: &[u8]) -> Result<DartModule, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<DartModule>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `DartModule` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_dart_module_unchecked`.
pub fn root_as_dart_module_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<DartModule<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<DartModule<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `DartModule` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_dart_module_unchecked`.
pub fn size_prefixed_root_as_dart_module_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<DartModule<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<DartModule<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a DartModule and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `DartModule`.
pub unsafe fn root_as_dart_module_unchecked(buf: &[u8]) -> DartModule {
  flatbuffers::root_unchecked::<DartModule>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed DartModule and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `DartModule`.
pub unsafe fn size_prefixed_root_as_dart_module_unchecked(buf: &[u8]) -> DartModule {
  flatbuffers::size_prefixed_root_unchecked::<DartModule>(buf)
}
#[inline]
pub fn finish_dart_module_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<DartModule<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_dart_module_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<DartModule<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod javascript_engine_dart_wrapper

