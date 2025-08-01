// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod congee_flat {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_NODE_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_NODE_TYPE: u8 = 7;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_NODE_TYPE: [NodeType; 8] = [
  NodeType::N4_INTERNAL,
  NodeType::N16_INTERNAL,
  NodeType::N48_INTERNAL,
  NodeType::N256_INTERNAL,
  NodeType::N4_LEAF,
  NodeType::N16_LEAF,
  NodeType::N48_LEAF,
  NodeType::N256_LEAF,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct NodeType(pub u8);
#[allow(non_upper_case_globals)]
impl NodeType {
  pub const N4_INTERNAL: Self = Self(0);
  pub const N16_INTERNAL: Self = Self(1);
  pub const N48_INTERNAL: Self = Self(2);
  pub const N256_INTERNAL: Self = Self(3);
  pub const N4_LEAF: Self = Self(4);
  pub const N16_LEAF: Self = Self(5);
  pub const N48_LEAF: Self = Self(6);
  pub const N256_LEAF: Self = Self(7);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 7;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::N4_INTERNAL,
    Self::N16_INTERNAL,
    Self::N48_INTERNAL,
    Self::N256_INTERNAL,
    Self::N4_LEAF,
    Self::N16_LEAF,
    Self::N48_LEAF,
    Self::N256_LEAF,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::N4_INTERNAL => Some("N4_INTERNAL"),
      Self::N16_INTERNAL => Some("N16_INTERNAL"),
      Self::N48_INTERNAL => Some("N48_INTERNAL"),
      Self::N256_INTERNAL => Some("N256_INTERNAL"),
      Self::N4_LEAF => Some("N4_LEAF"),
      Self::N16_LEAF => Some("N16_LEAF"),
      Self::N48_LEAF => Some("N48_LEAF"),
      Self::N256_LEAF => Some("N256_LEAF"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for NodeType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for NodeType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for NodeType {
    type Output = NodeType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for NodeType {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for NodeType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for NodeType {}
// struct Child, aligned to 2
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Child(pub [u8; 4]);
impl Default for Child { 
  fn default() -> Self { 
    Self([0; 4])
  }
}
impl core::fmt::Debug for Child {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Child")
      .field("key", &self.key())
      .field("node_index", &self.node_index())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Child {}
impl<'a> flatbuffers::Follow<'a> for Child {
  type Inner = &'a Child;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Child>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Child {
  type Inner = &'a Child;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Child>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Child {
    type Output = Child;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Child as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(2)
    }
}

impl<'a> flatbuffers::Verifiable for Child {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Child {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    key: u8,
    node_index: u16,
  ) -> Self {
    let mut s = Self([0; 4]);
    s.set_key(key);
    s.set_node_index(node_index);
    s
  }

  pub fn key(&self) -> u8 {
    let mut mem = core::mem::MaybeUninit::<<u8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_key(&mut self, x: u8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn node_index(&self) -> u16 {
    let mut mem = core::mem::MaybeUninit::<<u16 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[2..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u16 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_node_index(&mut self, x: u16) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[2..].as_mut_ptr(),
        core::mem::size_of::<<u16 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum NodeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Node<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Node<'a> {
  type Inner = Node<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Node<'a> {
  pub const VT_NODE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_PREFIX: flatbuffers::VOffsetT = 6;
  pub const VT_CHILDREN: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Node { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args NodeArgs<'args>
  ) -> flatbuffers::WIPOffset<Node<'bldr>> {
    let mut builder = NodeBuilder::new(_fbb);
    if let Some(x) = args.children { builder.add_children(x); }
    if let Some(x) = args.prefix { builder.add_prefix(x); }
    builder.add_node_type(args.node_type);
    builder.finish()
  }


  #[inline]
  pub fn node_type(&self) -> NodeType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<NodeType>(Node::VT_NODE_TYPE, Some(NodeType::N4_INTERNAL)).unwrap()}
  }
  #[inline]
  pub fn prefix(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Node::VT_PREFIX, None)}
  }
  #[inline]
  pub fn children(&self) -> Option<flatbuffers::Vector<'a, Child>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Child>>>(Node::VT_CHILDREN, None)}
  }
}

impl flatbuffers::Verifiable for Node<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<NodeType>("node_type", Self::VT_NODE_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("prefix", Self::VT_PREFIX, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Child>>>("children", Self::VT_CHILDREN, false)?
     .finish();
    Ok(())
  }
}
pub struct NodeArgs<'a> {
    pub node_type: NodeType,
    pub prefix: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub children: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Child>>>,
}
impl<'a> Default for NodeArgs<'a> {
  #[inline]
  fn default() -> Self {
    NodeArgs {
      node_type: NodeType::N4_INTERNAL,
      prefix: None,
      children: None,
    }
  }
}

pub struct NodeBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> NodeBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_node_type(&mut self, node_type: NodeType) {
    self.fbb_.push_slot::<NodeType>(Node::VT_NODE_TYPE, node_type, NodeType::N4_INTERNAL);
  }
  #[inline]
  pub fn add_prefix(&mut self, prefix: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_PREFIX, prefix);
  }
  #[inline]
  pub fn add_children(&mut self, children: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Child>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_CHILDREN, children);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> NodeBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    NodeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Node<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Node<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Node");
      ds.field("node_type", &self.node_type());
      ds.field("prefix", &self.prefix());
      ds.field("children", &self.children());
      ds.finish()
  }
}
pub enum CongeeFlatOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct CongeeFlat<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CongeeFlat<'a> {
  type Inner = CongeeFlat<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> CongeeFlat<'a> {
  pub const VT_NODES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    CongeeFlat { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args CongeeFlatArgs<'args>
  ) -> flatbuffers::WIPOffset<CongeeFlat<'bldr>> {
    let mut builder = CongeeFlatBuilder::new(_fbb);
    if let Some(x) = args.nodes { builder.add_nodes(x); }
    builder.finish()
  }


  #[inline]
  pub fn nodes(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node>>>>(CongeeFlat::VT_NODES, None)}
  }
}

impl flatbuffers::Verifiable for CongeeFlat<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Node>>>>("nodes", Self::VT_NODES, false)?
     .finish();
    Ok(())
  }
}
pub struct CongeeFlatArgs<'a> {
    pub nodes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node<'a>>>>>,
}
impl<'a> Default for CongeeFlatArgs<'a> {
  #[inline]
  fn default() -> Self {
    CongeeFlatArgs {
      nodes: None,
    }
  }
}

pub struct CongeeFlatBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> CongeeFlatBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_nodes(&mut self, nodes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Node<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(CongeeFlat::VT_NODES, nodes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> CongeeFlatBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    CongeeFlatBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<CongeeFlat<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for CongeeFlat<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("CongeeFlat");
      ds.field("nodes", &self.nodes());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `CongeeFlat`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_congee_flat_unchecked`.
pub fn root_as_congee_flat(buf: &[u8]) -> Result<CongeeFlat, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<CongeeFlat>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `CongeeFlat` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_congee_flat_unchecked`.
pub fn size_prefixed_root_as_congee_flat(buf: &[u8]) -> Result<CongeeFlat, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<CongeeFlat>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `CongeeFlat` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_congee_flat_unchecked`.
pub fn root_as_congee_flat_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<CongeeFlat<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<CongeeFlat<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `CongeeFlat` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_congee_flat_unchecked`.
pub fn size_prefixed_root_as_congee_flat_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<CongeeFlat<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<CongeeFlat<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a CongeeFlat and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `CongeeFlat`.
pub unsafe fn root_as_congee_flat_unchecked(buf: &[u8]) -> CongeeFlat {
  flatbuffers::root_unchecked::<CongeeFlat>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed CongeeFlat and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `CongeeFlat`.
pub unsafe fn size_prefixed_root_as_congee_flat_unchecked(buf: &[u8]) -> CongeeFlat {
  flatbuffers::size_prefixed_root_unchecked::<CongeeFlat>(buf)
}
#[inline]
pub fn finish_congee_flat_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<CongeeFlat<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_congee_flat_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<CongeeFlat<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod CongeeFlat

