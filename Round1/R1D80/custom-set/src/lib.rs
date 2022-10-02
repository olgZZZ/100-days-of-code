#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T> CustomSet<T> 
where T: PartialEq + Clone + Ord,
{
    pub fn new(input: &[T]) -> Self {
        let v = input.to_vec();
        let mut custom_set = CustomSet { elements: v };
        custom_set.elements.sort();
        custom_set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.elements.contains(&element) {
            self.elements.push(element);
            self.elements.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| other.elements.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| !other.elements.contains(e))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let v: Vec<T> = self.elements
            .iter()
            .filter(|e| other.contains(e))
            .cloned()
            .collect();
        CustomSet::new(&v)
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let v: Vec<T> = self.elements
            .iter()
            .filter(|e| !other.contains(e))
            .cloned()
            .collect();
        CustomSet::new(&v)
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut v = self.elements.clone();
        for e in other.elements.iter() {
            if !v.contains(e) {
                v.push(e.clone());
            }
        }
        v.sort();
        CustomSet::new(&v)
    }
}
