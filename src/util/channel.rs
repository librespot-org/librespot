macro_rules! implement_sender {
  (name => $name:ident, 
   wrap => $wrap_type:ident,
   with => $with_type:ident,
   variant => $variant:ident) =>  {
    pub struct $name {
      wrapped_sender: ::std::sync::mpsc::Sender<$with_type>,
    }

    impl $name {
      pub fn create(sender: ::std::sync::mpsc::Sender<$with_type>) -> $name {
        $name {
          wrapped_sender: sender
        }
      }
      pub fn send(&self, t: $wrap_type) -> Result<(), ::std::sync::mpsc::SendError<$wrap_type>> {
        let wrapped = self.wrap(t);
            let result = self.wrapped_sender.send(wrapped);
            result.map_err(|senderror| { 
                let ::std::sync::mpsc::SendError(z) = senderror;
                ::std::sync::mpsc::SendError(self.unwrap(z))
            })
      }
      fn wrap(&self, d: $wrap_type) -> $with_type {
        $with_type::$variant(d)
      }
      fn unwrap(&self, msg: $with_type) -> $wrap_type {
        let d = match msg {
                $with_type::$variant(d) => d,
                _ => unreachable!()
        };
        d
      }
    }

    impl Clone for $name {
        fn clone(&self) -> $name {
            $name {
                wrapped_sender: self.wrapped_sender.clone()
            }
        }
    }
  }
}