use crate::triangle::Triangle2D;
use ultraviolet::Vec2;

#[derive(Debug)]
pub struct AABB {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl AABB {
    pub fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn from_points(min: Vec2, max: Vec2) -> Self {
        Self {
            min_x: min.x,
            max_x: max.x,
            min_y: min.y,
            max_y: max.y,
        }
    }

    pub fn point_in_aabb(&self, point: &Vec2) -> bool {
        point.x > self.min_x && point.x < self.max_x && point.y > self.min_y && point.y < self.max_y
    }

    fn intersects(&self, other: &Self) -> bool {
        self.max_x > other.min_x && other.max_x > self.min_x &&
        self.max_y > other.min_y && other.max_y > self.min_y
    }

    /// Returns the intersection of `self` and `other` as a new `AABB`.
    /// Result is `None` if `self` and `other` do not intersect.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(
                Self {
                    min_x: f32::max(self.min_x, other.min_x),
                    max_x: f32::min(self.max_x, other.max_x),
                    min_y: f32::max(self.min_y, other.min_y),
                    max_y: f32::min(self.max_y, other.max_y),
                }
            )
        } else {
            None
        }
    }
}

impl From<&Triangle2D> for AABB {
    fn from(value: &Triangle2D) -> Self {
        AABB {
            min_x: value.v0.x.min(value.v1.x).min(value.v2.x),
            max_x: value.v0.x.max(value.v1.x).max(value.v2.x),
            min_y: value.v0.y.min(value.v1.y).min(value.v2.y),
            max_y: value.v0.y.max(value.v1.y).max(value.v2.y),
        }
    }
}

impl IntoIterator for AABB {
    type Item = Vec2;

    type IntoIter = AABBIter;

    fn into_iter(self) -> Self::IntoIter {
        return AABBIter::new(self);
    }
}

pub struct AABBIter {
    inner: AABB,
    x: f32,
    y: f32,
    ran_size_check: bool,
}

impl AABBIter {
    fn new(inner: AABB) -> Self {
        let x = inner.min_x.floor();
        let y = inner.min_y.floor();
        Self { inner, x, y, ran_size_check: false }
    }
}

impl Iterator for AABBIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.ran_size_check {
            if self.inner.max_x - self.inner.min_x < 1. || self.inner.max_y - self.inner.min_y < 1. {
                return None;
            } else {
                self.ran_size_check = true;
            }
        }

        if self.x < self.inner.max_x {
            if self.y < self.inner.max_y {
                let x = self.x;
                self.x += 1.;
                return Some(Vec2::new(x, self.y));
            } else {
                return None;
            }
        } else {
            self.x = self.inner.min_x.floor();
            self.y += 1.;
            return self.next();
        }
    }
}
