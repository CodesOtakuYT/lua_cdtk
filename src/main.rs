extern crate colorize;
use colorize::AnsiColor;

use mlua::prelude::*;
use num::{self};
use std::env;
use std::fs::read_to_string;

#[derive(Clone)]
struct Vector<
    T: Clone
        + Copy
        + num::Num
        + for<'lua> ToLua<'lua>
        + for<'lua> FromLua<'lua>
        + std::fmt::Debug
        + std::cmp::PartialOrd
        + std::ops::Neg<Output = T>,
> {
    data: Vec<T>,
}

impl<
        T: Clone
            + Copy
            + num::Num
            + for<'lua> ToLua<'lua>
            + for<'lua> FromLua<'lua>
            + std::fmt::Debug
            + std::cmp::PartialOrd
            + std::ops::Neg<Output = T>,
    > Vector<T>
{
    fn new() -> Self {
        let data = Vec::new();
        Vector { data }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn sum(&self) -> T {
        if self.data.len() == 0 {
            return T::zero();
        }
        let mut result: T = self.data[0];
        for item in self.data.iter().skip(1) {
            result = result + *item;
        }
        result
    }

    fn product(&self) -> T {
        if self.data.len() == 0 {
            return T::one();
        }
        let mut result: T = self.data[0];
        for item in self.data.iter().skip(1) {
            result = result * *item;
        }
        result
    }

    fn max(&self) -> T {
        if self.data.len() == 0 {
            return T::zero();
        }
        let mut result: T = self.data[0];
        for item in self.data.iter().skip(1) {
            if result < *item {
                result = *item;
            }
        }
        result
    }

    fn min(&self) -> T {
        if self.data.len() == 0 {
            return T::zero();
        }
        let mut result: T = self.data[0];
        for item in self.data.iter().skip(1) {
            if result > *item {
                result = *item;
            }
        }
        result
    }

    fn negate(&mut self) {
        // TODO: Figure out a way to avoid cloning the vector without invoking the memory checker
        for (index, &item) in self.data.clone().iter().enumerate() {
            self.data[index] = -item;
        }
    }

    fn range(&mut self, min: T, max: T, step: T) {
        let dif = max - min;
        let mut step = step;

        if step < T::zero() {
            step = -step;
        }

        if dif > T::zero() {
            let mut i = min;
            while i <= max {
                self.push(i);
                i = i + step;
            }
        } else if dif < T::zero() {
            let (min, max) = (max, min);
            let mut i = max;
            while i >= min {
                self.push(i);
                i = i - step;
            }
        } else {
            self.push(min);
        }
    }

    fn fill(&mut self, value: T, count: T) {
        let mut i = T::zero();
        while i < count {
            self.push(value);
            i = i + T::one();
        }
    }

    fn tostring(&self) -> String {
        format!("{:?}", self.data)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn unm(&self) -> Self {
        //self.data.iter().map(|x| -*x).collect()
        let mut vec = Self::new();
        for value in self.data.iter() {
            vec.push(-*value)
        }
        vec
    }

    fn add(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            vec.push(
                *self.data.get(i).unwrap_or(&T::zero()) + *other.data.get(i).unwrap_or(&T::zero()),
            )
        }
        vec
    }

    fn sub(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            vec.push(
                *self.data.get(i).unwrap_or(&T::zero()) - *other.data.get(i).unwrap_or(&T::zero()),
            )
        }
        vec
    }

    fn mul(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            vec.push(
                *self.data.get(i).unwrap_or(&T::one()) * *other.data.get(i).unwrap_or(&T::one()),
            )
        }
        vec
    }

    fn div(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            vec.push(
                *self.data.get(i).unwrap_or(&T::one()) / *other.data.get(i).unwrap_or(&T::one()),
            )
        }
        vec
    }

    fn rem(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            vec.push(
                *self.data.get(i).unwrap_or(&T::one()) % *other.data.get(i).unwrap_or(&T::one()),
            )
        }
        vec
    }

    fn eq(&self, other: Self) -> bool {
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            if *self.data.get(i).unwrap_or(&T::zero()) != *other.data.get(i).unwrap_or(&T::zero()) {
                return false;
            }
        }
        true
    }

    fn lt(&self, other: Self) -> bool {
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            if *self.data.get(i).unwrap_or(&T::zero()) >= *other.data.get(i).unwrap_or(&T::zero()) {
                return false;
            }
        }
        true
    }

    fn le(&self, other: Self) -> bool {
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            if *self.data.get(i).unwrap_or(&T::zero()) > *other.data.get(i).unwrap_or(&T::zero()) {
                return false;
            }
        }
        true
    }

    fn pow(&self, other: Self) -> Self {
        let mut vec = Self::new();
        for i in 0..std::cmp::max(self.data.len(), other.data.len()) {
            let mut result = T::one();
            let mut j = T::zero();

            while j < *other.data.get(i).unwrap_or(&T::zero()) {
                result = result * (*self.data.get(i).unwrap_or(&T::one()));
                j = j + T::one();
            }
            vec.push(result)
        }
        vec
    }

    fn concat(&self, other: Self) -> Self {
        let mut vec = Self::new();
        vec.data.extend(self.data.iter().chain(other.data.iter()));
        vec
    }

    fn index(&self, index: usize) -> T {
        self.data[index]
    }

    fn newindex(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    fn scale(&self, scalar: T) -> Self {
        let mut vec = Self::new();
        for value in self.data.iter() {
            vec.push(*value * scalar);
        }
        vec
    }

    // fn table(&self, lua_ctx: &Lua) -> LuaTable {
        
    // }
}

impl<
        T: Clone
            + Copy
            + num::Num
            + for<'lua> ToLua<'lua>
            + for<'lua> FromLua<'lua>
            + std::fmt::Debug
            + std::cmp::PartialOrd
            + std::ops::Neg<Output = T>
            + 'static,
    > LuaUserData for Vector<T>
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("sum", |_, this, ()| Ok(this.sum()));
        methods.add_method("product", |_, this, ()| Ok(this.product()));
        methods.add_method("max", |_, this, ()| Ok(this.max()));
        methods.add_method("min", |_, this, ()| Ok(this.min()));
        methods.add_method("clone", |_, this, ()| Ok(this.clone()));
        methods.add_method("table", |lua_ctx, this, ()| Ok(
            {
                let t = lua_ctx.create_table().unwrap();
                for (i, value) in this.data.iter().enumerate() {
                    t.set(i+1, *value).unwrap();
                }
                t
            }
        ));
        methods.add_method_mut("push", |_, this, value| Ok(this.push(value)));
        methods.add_method_mut("pop", |_, this, ()| Ok(this.pop()));
        methods.add_method_mut("negate", |_, this, ()| Ok(this.negate()));
        methods.add_method_mut("range", |_, this, (min, max, step)| {
            Ok(this.range(min, max, step))
        });
        methods.add_method_mut(
            "fill",
            |_, this, (value, count)| Ok(this.fill(value, count)),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(this.tostring()));
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, ()| Ok(this.len()));
        methods.add_meta_method(LuaMetaMethod::Unm, |_, this, ()| Ok(this.unm()));
        methods.add_meta_method(LuaMetaMethod::Add, |_, this, other| Ok(this.add(other)));
        methods.add_meta_method(LuaMetaMethod::Sub, |_, this, other| Ok(this.sub(other)));
        methods.add_meta_method(LuaMetaMethod::Mul, |_, this, other| Ok(this.mul(other)));
        methods.add_meta_method(LuaMetaMethod::Div, |_, this, other| Ok(this.div(other)));
        methods.add_meta_method(LuaMetaMethod::Mod, |_, this, other| Ok(this.rem(other)));
        methods.add_meta_method(LuaMetaMethod::Pow, |_, this, other| Ok(this.pow(other)));
        methods.add_meta_method(LuaMetaMethod::Concat, |_, this, other| {
            Ok(this.concat(other))
        });
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other| Ok(this.eq(other)));
        methods.add_meta_method(LuaMetaMethod::Lt, |_, this, other| Ok(this.lt(other)));
        methods.add_meta_method(LuaMetaMethod::Le, |_, this, other| Ok(this.le(other)));

        methods.add_meta_method(LuaMetaMethod::Index, |_, this, index| Ok(this.index(index)));
        methods.add_meta_method_mut(LuaMetaMethod::NewIndex, |_, this, (index, value)| {
            Ok(this.newindex(index, value))
        });
        methods.add_meta_method(
            LuaMetaMethod::Call,
            |_, this, scalar| Ok(this.scale(scalar)),
        );
    }
}

fn main() {
    let lua = Lua::new();
    let cdtk_table = lua.create_table().unwrap();

    cdtk_table
        .set(
            "vec_f64",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<f64>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_f32",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<f32>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_i128",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<i128>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_i64",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<i64>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_i32",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<i32>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_i16",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<i16>::new()))
                .unwrap(),
        )
        .unwrap();

    cdtk_table
        .set(
            "vec_i8",
            lua.create_function(|_lua_ctx, ()| Ok(Vector::<i8>::new()))
                .unwrap(),
        )
        .unwrap();

    let globals = lua.globals();
    globals.set("CDTK", cdtk_table).unwrap();

    let filename = env::args().skip(1).next().expect(&format!(
        "{}",
        "filename argument is required".to_uppercase().bold().red()
    ));

    lua.load(&read_to_string(filename).expect(&format!(
        "{}",
        "Failed to load script file".to_uppercase().bold().red()
    )))
    .exec()
    .expect(&format!(
        "{}",
        "Failed to execute script".to_uppercase().bold().red()
    ))
}
