pub mod merkle_tree;
pub mod merkle_trace;
pub(super) mod node;
pub enum TreeShape{
    FullCopyExtend,
    FullNullExtend,
    PartialCopyExtend,
    PartialNullExtend,
}

