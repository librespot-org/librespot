/// Creates an implentation of sender which can be used to share an async channel for multiple message types.
///
/// # Examples
///
/// ```
/// struct D;
/// struct A;
///
/// enum Msg {
///   FirstType(D),
///   SecondType(A)
/// }
///
/// fn main() {
///   let (tx, rx) = channel::<Msg>();
///
///   let d_sender = DSender::create(tx.clone());
///   let a_sender = ASender::create(tx.clone());
///   subscribe(d_sender.clone());
///   subscribe2(d_sender.clone());
///   subscribe3(a_sender.clone());
 //
///   let mut i = 0;
 //
///   for m in rx {
///     i += 1;
///     match m {
///       Msg::FirstType(_) => println!("m: D {}", i),
///       Msg::SecondType(_) => println!("m: A {}", i)
///     };
///   }
/// }
///
/// fn subscribe(sender: DSender) {
///   thread::spawn(move|| {
///       sender.send(D).unwrap();
///   });
/// }
/// fn subscribe2(sender: DSender) {
///   thread::spawn(move|| {
///       thread::sleep(time::Duration::from_millis(10));
///       sender.send(D).unwrap();
///   });
/// }
/// fn subscribe3(sender: ASender) {
///   thread::spawn(move|| {
///       sender.send(A).unwrap();
///   });
/// }
/// implement_sender!(name => DSender, 
///                   wrap => D, 
///                   with => Msg, 
///                   variant => FirstType)
/// implement_sender!(name => ASender, 
///                   wrap => A, 
///                   with => Msg, 
///                   variant => SecondType)
/// ```
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