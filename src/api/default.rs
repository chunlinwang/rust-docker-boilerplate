use actix_web::{get, Responder, web, HttpResponse, dev::HttpResponseBuilder};
use actix_identity::Identity;

#[get("/")]
pub async fn index(id: Identity) -> impl Responder {
    // access request identity
    if let Some(id) = id.identity() {
        println!("Welcome! {:?}", id);
    } else {
        println!("Welcome Anonymous!");
    }

    web::Json(
        [1, 2, 3]
        )
}

#[get("/jwks.json")]
pub async fn jwks() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .header("type", "jwks")
        .body("{\"keys\":[{\"p\":\"wQbcfF0EJCAHpeeI2ra5PQ5r-C68XjMlx194M93dR_AZKj1JRrd472coyUFw6zuB640k3Zb7C-6Peo5rYQ88a1FmWOmgzzo8tW8YYyBwdCAzEJhiQBYyKJ1ej50vu98O-lFID-_XkWXU24_a8M2rYY2R62YHJ3MyPbPvHHStY6k\",\"kty\":\"RSA\",\"q\":\"rqefqi7r3yM1eeTUHP0Rl0amTW24vxJwk2QjU7CbhRwGrOTZmBN86O_j1RRfCXJypbqPYLNWquXPrQntQUzUv54CcRYX_RodKtEyMwyTFRbChGn44CugOObvM4Q6w_PK9D9To2zZ8G75qkoTak53kKnauYmEJFz5VHSoZOFuZg8\",\"d\":\"AdvU4sP523AiagJ43LlmRtCIKtQRXBEdILrpDbY1El-v43V3ltHW95tnlJ0dUUJbGiy10xoj6BrFv8gJ7VW-8InsT-5tfmzaoGb7qHSSqYaBe32ZPf22JUyo7Ky2ofvXKS62ijZNCA3Jft32UIJOXTJUrpNGxFADugH1MpIfvU2IdTlFcz4QfCD6kPOobuqlgjb4cO08Kacm9HmbXnXD9pc52kfwY5NJytfpI8zZhXTCeJmqoU-P9-Ptltproia3PFF8DxzbWkd6F_v_FifQ2WvBwkzoAV3RBMNUhk74hoqu-EzahzUGC9tYl67EeZzvccrhx8uLfN0b9mGMxNVkcQ\",\"e\":\"AQAB\",\"use\":\"sig\",\"kid\":\"COJa2EDCWCuIz5EtLrvgrt--t4dTlgIebC22kOaNQes\",\"qi\":\"o5eS26oKlvzsHvOUJyN4DkPo5ckZiPsqXGLSiHS6Ix8RuR6Hnm4k922GvjrqWnVfgH9JAczsZybvK5poJUjS0RafQEA0EDJaUMrh1Up8Knb83HXiUcHvG7OO7wpMgLLd7qbyzfx6FuyU5fmPa86ReFqUrP0YV5OEq-TOJYqA-NI\",\"dp\":\"d-jTP05pLCfZmG5kzbmFCTDUUbB2w-3yIhBkk5PXysCUYXEA_DUto0rqQ1ur0fV9l95ucFlCSv8_klq2yey1E9XVRpablyAOZodFFw8nzHvUPRLstfMT5bcc7wouhP7kRwwHQB82QtSHjn2m_MBHHqrbYuhWuWMOatZES-EPdUE\",\"alg\":\"PS512\",\"dq\":\"BoIEdwjsjRH4yiP2q2Xao_Jj-_qcqbH8M-HFVzb7dIhIz2RvnzUKsHtP1_68U9PJ5db_xLCqCBAsHtyB1K8SNlmi6afTGQFCGPhWkIRlbqG8ecahGWlGvRgNzbOdpPOOmm9JpItHy6e28dt0wb9OwTszBTrUi65TNAxsweWMgY0\",\"n\":\"g7ENufE6O8byepJebasJMTWvkEFd25lgi374syJCB2fbO1MKUcMN9iPA54-eIayE_XURYj0qaVsDMpZ5mYJtrKiA7al-KQ5n2s56K8qeVQLFDMDTVB8I2_fCYDYNei5CJrYHc6PoTA7R9nCws4KYFB5o3WTVit8Tp-ds6WUaIh9CgL7COeIZdOgTUzSq7EXTvYqQ3Ex0sinB5CTB-IHGOh5NcuQ7XzZuYO4rrYqULRB_8au6RWQOr8-mkOBhNAiyAH2mdm1fUzJnKMq-n9dBmacIbsCA67N9oS7YVz5y9Tobq_VSg5eP6awNeFXoFZugxlI79J2j71UJTLZER3ws5w\"}]}")
}
