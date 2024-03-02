pub struct NavLinks {
    pub href: String,
    pub label: String,
}
impl NavLinks {
    pub fn iter() -> Vec<NavLinks> {
        let mut m = Vec::new();
        m.push(("#home", "Home"));
        m.push(("#about-us", "About Us"));
        m.push(("#products", "Products"));
        m.push(("#contact-us", "Contact Us"));
        let mut result = Vec::new();
        for (link, label) in m {
            result.push(NavLinks {
                href: link.to_string(),
                label: label.to_string(),
            })
        }
        result
    }
}
pub struct Statistics {
    pub value: String,
    pub label: String,
}
impl Statistics {
    pub fn iter() -> Vec<Statistics> {
        let mut m = Vec::new();
        m.push(("1k+", "Brands"));
        m.push(("500+", "Shops"));
        m.push(("250k+", "Customers"));
        let mut result = Vec::new();
        for (value, label) in m {
            result.push(Statistics {
                value: value.to_string(),
                label: label.to_string(),
            })
        }
        result
    }
}
#[derive(Clone, PartialEq)]
pub struct Shoes {
    pub thumbnail: String,
    pub big_shoe: String,
}
impl Shoes {
    pub fn iter() -> Vec<Shoes> {
        let mut m = Vec::new();
        m.push(("images/thumbnail-shoe1.svg", "images/big-shoe1.png"));
        m.push(("images/thumbnail-shoe2.svg", "images/big-shoe2.png"));
        m.push(("images/thumbnail-shoe3.svg", "images/big-shoe3.png"));
        let mut result = Vec::new();
        for (thumbnail, big_shoe) in m {
            result.push(Shoes {
                thumbnail: thumbnail.to_string(),
                big_shoe: big_shoe.to_string(),
            })
        }
        result
    }
}
#[derive(Clone, PartialEq)]
pub struct Product {
    pub img_url: String,
    pub name: String,
    pub price: String,
}
impl Product {
    pub fn iter() -> Vec<Product> {
        vec![
            Product {
                img_url: "images/shoe4.svg".to_string(),
                name: "Nike Air Jordan-01".to_string(),
                price: "$200.20".to_string(),
            },
            Product {
                img_url: "images/shoe5.svg".to_string(),
                name: "Nike Air Jordan-10".to_string(),
                price: "$210.20".to_string(),
            },
            Product {
                img_url: "images/shoe6.svg".to_string(),
                name: "Nike Air Jordan-100".to_string(),
                price: "$220.20".to_string(),
            },
            Product {
                img_url: "images/shoe7.svg".to_string(),
                name: "Nike Air Jordan-001".to_string(),
                price: "$230.20".to_string(),
            },
        ]
    }
}
#[derive(Clone, PartialEq)]
pub struct Service {
    pub img_url: String,
    pub label: String,
    pub subtext: String,
}
impl Service {
    pub fn iter() -> Vec<Service> {
        vec![
            Service {
                img_url: "icons/truck-fast.svg".to_string(),
                label: "Free shipping".to_string(),
                subtext: "Enjoy seamless shopping with our complimentary shipping service."
                    .to_string(),
            },
            Service {
                img_url: "icons/shield-tick.svg".to_string(),
                label: "Secure Payment".to_string(),
                subtext: "Experience worry-free transactions with our secure payment options."
                    .to_string(),
            },
            Service {
                img_url: "icons/support.svg".to_string(),
                label: "Love to help you".to_string(),
                subtext: "Our dedicated team is here to assist you every step of the way."
                    .to_string(),
            },
        ]
    }
}
#[derive(Clone, PartialEq)]
pub struct Review {
    pub img_url: String,
    pub customer_name: String,
    pub rating: f64,
    pub feedback: String,
}
impl Review {
    pub fn iter() -> Vec<Review> {
        vec![Review {
        img_url: "images/customer1.jpeg".to_string(),
        customer_name: "Morich Brown".to_string(),
        rating: 4.5,
        feedback: "The attention to detail and the quality of the product exceeded my expectations. Highly recommended!".to_string(),
    },
    Review {
        img_url: "images/customer2.svg".to_string(),
        customer_name: "Lota Mongeskar".to_string(),
        rating: 4.5,
        feedback: "The product not only met but exceeded my expectations. I'll definitely be a returning customer!".to_string(),
    }
]
    }
}
pub struct LinkStruct {
    pub name: String,
    pub link: String,
}
pub struct FooterLink {
    pub title: String,
    pub links: Vec<LinkStruct>,
}

impl FooterLink {
    pub fn iter() -> Vec<FooterLink> {
        vec![
            FooterLink {
                title: "Products".to_string(),
                links: vec![
                    LinkStruct {
                        name: "Air Force 1".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Air Max 1".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Air Jordan 1".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Air Force 2".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Nike Waffle Racer".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Nike Cortez".to_string(),
                        link: "/".to_string(),
                    },
                ],
            },
            FooterLink {
                title: "Help".to_string(),
                links: vec![
                    LinkStruct {
                        name: "About us".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "FAQs".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "How it works".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Privacy policy".to_string(),
                        link: "/".to_string(),
                    },
                    LinkStruct {
                        name: "Payment policy".to_string(),
                        link: "/".to_string(),
                    },
                ],
            },
            FooterLink {
                title: "Get in touch".to_string(),
                links: vec![
                    LinkStruct {
                        name: "customer@nike.com".to_string(),
                        link: "mailto:customer@nike.com".to_string(),
                    },
                    LinkStruct {
                        name: "+92554862354".to_string(),
                        link: "tel:+92554862354".to_string(),
                    },
                ],
            },
        ]
    }
}
pub struct SocialMedia {
    pub src: String,
    pub alt: String,
}
impl SocialMedia {
    pub fn iter() -> Vec<SocialMedia> {
        vec![
            SocialMedia {
                src: "icons/facebook.svg".to_string(),
                alt: "facebook logo".to_string(),
            },
            SocialMedia {
                src: "icons/twitter.svg".to_string(),
                alt: "twitter logo".to_string(),
            },
            SocialMedia {
                src: "icons/instagram.svg".to_string(),
                alt: "instagram logo".to_string(),
            },
        ]
    }
}
