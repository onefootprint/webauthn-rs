
Webauthn-rs
==========

Webauthn is a modern approach to hardware based authentication, consisting of
a user with an authenticator device, a browser or client that interacts with the
device, and a server that is able to generate challenges and verify the
authenticators validity.

Users are able to enroll their own tokens through a registration process to
be associated to their accounts, and then are able to login using the token
which performas a cryptographic authentication.

This library aims to provide useful functions and frameworks allowing you to
integrate webauthn into Rust web servers. This means the library implements the
Relying Party component of the FIDO2 workflow. We provide template and
example javascript and wasm bindings to demonstrate the browser interactions required.

Documentation
-------------

Our docs are available on [docs rs](https://docs.rs/webauthn-rs/latest/webauthn_rs/)

Demonstration
-------------

You can test this library via our [demonstration site](https://webauthn.firstyear.id.au/)

Or you can run the demonstration your self locally with:

    cd webauthn-rs-demo
    cargo run
    # For options
    cargo run -- --help

Known Supported Keys/Harwdare
-----------------------------

We have extensively tested a variety of keys and devices, not limited to:

* Yubico 5c / 5ci / FIPS / Bio
* TouchID / FaceID (iPhone, iPad, MacBook Pro)
* Windows Hello (TPM)
* Softtokens

If your key/browser combination don't work (generally due to missing crypto routines)
please conduct a [compatability test](https://webauthn.firstyear.id.au/compat_test) and then open
an issue so that we can resolve the issue!

Known BROKEN Keys/Harwdare
--------------------------

* Pixel 3a / Pixel 4 + Chrome - Does not send correct attestation certificates,
  and ignores requested algorithms. Not resolved.
* Windows 10 / Windows 11 + Firefox 98 - When aaguid is meant
  to be 16 bytes of 0, it emits a single 0 byte. This should be resolved as of 2022-04-17
* CTAP2.0/CTAP2.1 compliant devices - Incorrectly request UV on registration, but do not prompt for
  it during authentication, leading to possible social engineering and UV bypass attacks
* Windows Hello with TPM - Often use RSA-SHA1 signatures over attestation which may allow credential compromise/falsification

* BUG in safare, NOT Apple Passkeys (was, passkeys do not identify themself as a transferable credential, and should be considered to be floating.)

Standards Compliance
--------------------

This library has been carefully implemented to follow the w3c standard for webauthn level 3 processing
to ensure secure and correct behaviour. We support most major extensions and key types, but we do not claim
to be standards complaint because:

* We have enforced extra constraints in the library that go above and beyond the security guarantees the standard offers.
* We do not support certain esoteric options.
* We do not support all cryptographic primitive types (only limited to secure ones).
* A large number of advertised features in webauthn do not function in the real world.

This library has passed a security review performed by SUSE product security. Other security reviews
are welcome!

Feedback
--------

The current design of the traits and configuration is open to feedback on how it
can be improved - please use this library and contact the project on what can be
improved!

Why OpenSSL?
------------

A question I expect is why OpenSSL rather than some other pure-Rust cryptographic
providers. There are two major justfications.

The first is that if this library will be used in corporate or major deployments,
then cryptographic audits may have to be performed. It is much easier to point
toward OpenSSL which has already undergone much more review and auditing than
using a series of Rust crates which (while still great!) have not seen the same
level of scrutiny.

The second is that OpenSSL is the only library I have found that allows us to
reconstruct an EC public key from it's X/Y points or an RSA public key from it's
n/e for use with signature verification.
Without this, we are not able to parse authenticator credentials to perform authentication.

Resources
---------

* Specification: https://www.w3.org/TR/webauthn-3
* JSON details: https://fidoalliance.org/specs/fido-v2.0-rd-20180702/fido-server-v2.0-rd-20180702.html
* Write up on interactions: https://medium.com/@herrjemand/introduction-to-webauthn-api-5fd1fb46c285



