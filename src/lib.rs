/// the main trait that our communication shim will implement for each plugin
/// for example, if we have a POST request plugin, it can implement
/// the [`Mycelial`] trait. 
///
/// likewise, if we have a remote device (say a tasmota outlet) that
/// implements POST under the hood - we could also just implement
/// [`Mycelial`] for that plugin as well. 

pub trait Mycelial {
    fn send(&mut self);
    fn receive(&mut self);
}

pub trait AsyncMycelial {
    fn send(&self) -> impl core::future::Future<Output = ()>;
    fn receive(&self) -> impl core::future::Future<Output = ()>;
}

/// A node on the mycelia
pub struct Node {
    pub node_type: dyn Mycelial,
}


#[cfg(test)]
mod test {
    mod test_node {
        pub struct TestNode {
            name: String,
            pub send: bool,
            pub receive: bool,
        }
        impl TestNode {
            pub fn new(name: &str) -> Self {
                Self {
                    name: String::from(name),
                    send: false,
                    receive: false,
                }
            }

            pub fn name(&self) -> String {
                self.name.clone().into()
            }
        }
        
        use crate::Mycelial;
        impl Mycelial for TestNode {
            fn send(&mut self) {
                self.send = true;
            }

            fn receive(&mut self) {
                self.receive = true;
            }
        }
    }      

    use crate::Mycelial;
    
    #[test]
    fn test_send() {
        let name = "Test Node";
        let mut tn = test_node::TestNode::new(&name);
        assert_eq!(name, tn.name());
        
        tn.send();
        assert!(tn.send);

        tn.receive();
        assert!(tn.receive);
    }
}
