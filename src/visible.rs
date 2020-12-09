pub mod outer_mod {
    pub(self) fn outer_mod_fn() {
        println!("this fn only call in outer_mod");
    }
    pub mod inner_mod {
        pub(in crate::visible::outer_mod) fn outer_mod_visible_fn() {
            println!("inner_mod outer_mod_visible_fn()");
        }
        // outer_mod 内部可见
        pub(super) fn super_mod_visible_fn() {
            println!("inner_mod super_mod_visible_fn()");
            // 访问同一个模块的函数
            inner_mod_visible_fn();
            // 访问父类的函数
            super::outer_mod_fn();

            outer_mod_visible_fn();
        }

        //整个crate可见
        pub(crate) fn crate_visible_fn() {
            println!("inner_mod crate_visible_fn()");
        }

        //仅在inner_mod可见
        pub(self) fn inner_mod_visible_fn() {
            println!("inner_mod inner_mod_visible_fn()");
        }
    }

    pub fn foo() {
        println!("outer_mod foo()...");
        outer_mod_fn();
        inner_mod::outer_mod_visible_fn();
        inner_mod::super_mod_visible_fn();
        inner_mod::crate_visible_fn();
        // private function
        //inner_mod::inner_mod_visible_fn();
    }
}

pub fn bar() {
    outer_mod::inner_mod::crate_visible_fn();
    //该函数仅outer_mod可见
    //outer_mod::inner_mod::super_mod_visible_fn();
    //该函数仅outer_mod可见
    //outer_mod::inner_mod::outer_mod_visible_fn();
    outer_mod::foo();
}
