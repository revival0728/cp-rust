mod utils {
  // modified from https://crates.io/crates/fix_fn/
  macro_rules! recur_fn {
      (
          $($mov:ident)? |$self_arg:ident $(, $arg_name:ident : $arg_type:ty)* $(,)? |
              -> $ret_type:ty
          $body:block
      ) => {{
          trait hidefn {
              fn call(&self, $($arg_name : $arg_type ,)*) -> $ret_type;
          }
          struct hidefnimpl<f: fn(&dyn hidefn, $($arg_type ,)*) -> $ret_type>(f);
          impl<f: fn(&dyn hidefn, $($arg_type ,)*) -> $ret_type> hidefn for hidefnimpl<f> {
              #[inline]
              fn call(&self, $($arg_name : $arg_type ,)*) -> $ret_type {
                  self.0(self, $($arg_name ,)*)
              }
          }
          let inner = hidefnimpl(
              #[inline]
              $($mov)?
              |$self_arg, $($arg_name : $arg_type ,)*| -> $ret_type {
                  let $self_arg = |$($arg_name : $arg_type ),*| $self_arg.call($($arg_name ,)*);
                  {
                      $body
                  }
              }
          );
          #[inline]
          move |$($arg_name : $arg_type),*| -> $ret_type {
              inner.call($($arg_name),*)
          }
      }};
      (
          $($mov:ident)? |$($arg_name:ident $(: $arg_type:ty)?),* $(,)?|
          $body:expr
      ) => {
          compile_error!("closure passed to fix_fn needs return type!");
      };
      (
          $($mov:ident)? |$self_arg:ident : $self_type:ty $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? |
              -> $ret_type:ty
          $body:block
      ) => {
          compile_error!(concat!("First parameter ", stringify!($self_arg), " may not have type annotation!"));
      };
      (
          $($mov:ident)? |$self_arg:ident $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? |
              -> $ret_type:ty
          $body:block
      ) => {
          compile_error!("All parameters except first need to have an explicit type annotation!");
      };
  }

  pub(crate) use recur_fn;
}