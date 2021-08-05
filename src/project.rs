use std::collections::{BTreeMap, BTreeSet, HashMap};

use timeliner::{Timeline, TimelineItem};

type Id = String;

pub enum Interpolator {
    Linear,
    Custom(Id)
}

pub struct Interpolable<T> { // NOTE: end_valueの位置はinterpolatorに渡して補間してもらう
    pub start_value: T,
    pub values: BTreeMap<u32, T>,
    pub end_value: T,
    pub interpolator: Interpolator
}

pub enum ClipPropValue {
    Integer(Interpolable<u64>),
    Real(Interpolable<f64>)
}

pub struct ClipProp {
    pub name: String,
    pub id: Id,
    pub value: ClipPropValue
}

pub struct Clip {
    pub name: String,
    pub layer: u32,
    pub start: u32,
    pub end: u32,
    pub props: Vec<ClipProp>,
    // pub composite_mode: CompositeMode, // + 描画なしモードもここに(clip用)
    // pub clip_by: enum ClipBy{ Above / Object(Id) } NOTE: Objectによるclipの実装はたぶん面倒なのでAboveだけにしといたほうがいいかもしれない
}

impl TimelineItem for Clip {
    type Pos = u32;

    fn start(&self) -> Self::Pos {
        self.start
    }

    fn end(&self) -> Self::Pos {
        self.end
    }
}

pub struct Layer {
    pub name: String,
    pub clips: Timeline<Clip>
}

#[derive(Default)]
pub struct Sequence {
    pub layers: Vec<Layer>,
    pub clips: HashMap<Id, Clip>
}

pub struct Project {
    pub sequences: Sequence
}
