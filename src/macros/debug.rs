#[macro_export]
macro_rules! print_ast {
    ([ $($ast:tt),* ]) => {
        $(
            print_ast!($ast);
        )*
    };

    ({
        type: class,
        name: $name:tt,
        attributes: $attributes:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: $struct:tt,
        methods: $methods:tt
    }) => {
        println!("\nCLASS\n=====\nclass {} (pub:{} reopen:{}, attributes: {})", stringify!($name), stringify!($pub), stringify!($reopen), stringify!($attributes));
        println!("{}", format_ast_class_struct!({ name: $name, tuple: $struct }));
        println!("\nMETHODS\n=======\n{}\n\n", format_ast_methods!($methods));
    };

    ($($ast:tt)*) => {
        println!("Unimplemented: {}", stringify!($($ast)*));
    };
}

#[macro_export]
macro_rules! format_ast_class_struct {
    ({
        name: $name:tt,
        tuple: { $( $field:tt : $fieldty:ty ),* }
    }) => {
        {
            let mut s = String::new();
            $(
                s.push_str(&format!("struct {} {{ {}: {} }}", stringify!($name), stringify!($field), stringify!($fieldty)));
            )*
            s
        }
    };

    ({
        name: $name:tt,
        tuple: ()
    }) => {
        "No Struct"
    };
}

#[macro_export]
macro_rules! format_ast_methods {
    ([
        $($ast:tt),*
    ]) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format_ast_methods!($ast));
            )*
            v.join("\n")
        }
    };

    ({
        type: instance_method,
        name: $name:tt,
        self: $self:tt,
        args: $args:tt,
        ret: $ret:tt,
        body: $body:tt
    }) => {
        {
            let mut s = String::new();
            s.push_str(&format!("def {}", stringify!($name)));
            s.push_str(&format!("({})", format_ast_args!($args)));
            s.push_str(&format_ret!($ret));
            s.push_str(" ");
            s.push_str(&stringify!($body));
            s
        }
    };

    ({
        type: class_method,
        name: $name:tt,
        self: $self:tt,
        args: $args:tt,
        ret: $ret:ty,
        body: $body:tt
    }) => {
        {
            let mut s = String::new();
            s.push_str(&format!("def self.{}", stringify!($name)));
            s.push_str(&format!("({})", format_ast_args!($args)));
            s.push_str(&format_ret!($ret));
            s.push_str(" ");
            s.push_str(&stringify!($body));
            s
        }
    };


    ({
        type: initializer,
        name: $name:tt,
        self: $self:tt,
        args: $args:tt,
        ret: $ret:tt,
        body: $body:tt
    }) => {
        {
            let mut s = String::new();
            s.push_str(&format!("def initialize({})", format_ast_args!($args)));
            s.push_str(&format_ret!($ret));
            s.push_str(" ");
            s.push_str(&stringify!($body));
            s
        }
    };
}

#[macro_export]
macro_rules! format_ret {
    ($ret:ty) => { format!(" -> {}", stringify!($ret)) };
}

#[macro_export]
macro_rules! format_ast_args {
    ([ $($args:tt)* ]) => {
        format!("{}", stringify!($($args)*))
    }
}
