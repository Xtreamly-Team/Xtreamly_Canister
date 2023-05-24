(function() {var implementors = {
"base64":[["impl&lt;'e, E: <a class=\"trait\" href=\"base64/engine/trait.Engine.html\" title=\"trait base64::engine::Engine\">Engine</a>, R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"base64/read/struct.DecoderReader.html\" title=\"struct base64::read::DecoderReader\">DecoderReader</a>&lt;'e, E, R&gt;"]],
"binread":[],
"bytes":[["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/trait.Buf.html\" title=\"trait bytes::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"bytes/buf/struct.Reader.html\" title=\"struct bytes::buf::Reader\">Reader</a>&lt;B&gt;"]],
"digest":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"digest/core_api/struct.XofReaderCoreWrapper.html\" title=\"struct digest::core_api::XofReaderCoreWrapper\">XofReaderCoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.XofReaderCore.html\" title=\"trait digest::core_api::XofReaderCore\">XofReaderCore</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</span>"]],
"futures_util":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"futures_util/io/struct.AllowStdIo.html\" title=\"struct futures_util::io::AllowStdIo\">AllowStdIo</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</span>"]],
"ic_cdk":[["impl&lt;M: StableMemory_&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u64.html\">u64</a>&gt; + <a class=\"trait\" href=\"ic_cdk/api/stable/trait.StableMemory.html\" title=\"trait ic_cdk::api::stable::StableMemory\">StableMemory</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"ic_cdk/api/stable/struct.StableIO.html\" title=\"struct ic_cdk::api::stable::StableIO\">StableIO</a>&lt;M, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u64.html\">u64</a>&gt;"],["impl&lt;M: <a class=\"trait\" href=\"ic_cdk/api/stable/trait.StableMemory.html\" title=\"trait ic_cdk::api::stable::StableMemory\">StableMemory</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"ic_cdk/api/stable/struct.StableReader.html\" title=\"struct ic_cdk::api::stable::StableReader\">StableReader</a>&lt;M&gt;"],["impl&lt;M: <a class=\"trait\" href=\"ic_cdk/api/stable/trait.StableMemory.html\" title=\"trait ic_cdk::api::stable::StableMemory\">StableMemory</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"ic_cdk/api/stable/struct.BufferedStableReader.html\" title=\"struct ic_cdk::api::stable::BufferedStableReader\">BufferedStableReader</a>&lt;M&gt;"],["impl&lt;M: StableMemory_&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u32.html\">u32</a>&gt; + <a class=\"trait\" href=\"ic_cdk/api/stable/trait.StableMemory.html\" title=\"trait ic_cdk::api::stable::StableMemory\">StableMemory</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"ic_cdk/api/stable/struct.StableIO.html\" title=\"struct ic_cdk::api::stable::StableIO\">StableIO</a>&lt;M, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u32.html\">u32</a>&gt;"]],
"rand_core":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for dyn <a class=\"trait\" href=\"rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()