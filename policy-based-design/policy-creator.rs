//  Creatpr Policy  

pub trait Creator<T> {
    fn create(&self) -> T;
}

pub struct OpNewCreator;

impl<T: Default> Creator<T> for OpNewCreator {
    fn create(&self) -> T {
        T::default()
    }
}

// Prototype-based creator
pub struct PrototypeCreator<T: Clone> {
    prototype: T,
}

impl<T: Clone> PrototypeCreator<T> {
    pub fn new(prototype: T) -> Self {
        Self { prototype }
    }
}

impl<T: Clone> Creator<T> for PrototypeCreator<T> {
    fn create(&self) -> T {
        self.prototype.clone()
    }
}

impl<T: Clone> PrototypeCreator<T> {
    pub fn get_prototype(&self) -> &T {
        &self.prototype
    }
    pub fn set_prototype(&mut self, proto: T) {
        self.prototype = proto;
    }
}


pub struct WidgetManager<C, T>
where
    C: Creator<T>,
{
    creator: C,
    _marker: std::marker::PhantomData<T>,
}

// host
impl<C, T> WidgetManager<C, T>
where
    C: Creator<T>,
{
    pub fn new(creator: C) -> Self {
        Self {
            creator,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn create(&self) -> T {
        self.creator.create()
    }

    // Optional feature: only works if C is a PrototypeCreator<T>
    pub fn switch_prototype(&mut self, new_proto: T)
    where
        C: PrototypeCapable<T>,
    {
        self.creator.set_prototype(new_proto);
    }
}

pub trait PrototypeCapable<T> {
    fn get_prototype(&self) -> &T;
    fn set_prototype(&mut self, proto: T);
}

impl<T: Clone> PrototypeCapable<T> for PrototypeCreator<T> {
    fn get_prototype(&self) -> &T {
        &self.prototype
    }
    fn set_prototype(&mut self, proto: T) {
        self.prototype = proto;
    }
}
