(function() {var implementors = {
"arrayvec":[["impl&lt;const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayString.html\" title=\"struct arrayvec::ArrayString\">ArrayString</a>&lt;CAP&gt;"],["impl&lt;T, const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.slice.html\">[T]</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;"]],
"beef":[["impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;T&gt; for <a class=\"struct\" href=\"beef/generic/struct.Cow.html\" title=\"struct beef::generic::Cow\">Cow</a>&lt;'_, T, U&gt;<span class=\"where fmt-newline\">where\n    T: Beef + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    U: Capacity,</span>"]],
"bytes":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"bytes/struct.Bytes.html\" title=\"struct bytes::Bytes\">Bytes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"bytes/struct.BytesMut.html\" title=\"struct bytes::BytesMut\">BytesMut</a>"]],
"generic_array":[["impl&lt;T, N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.slice.html\">[T]</a>&gt; for <a class=\"struct\" href=\"generic_array/struct.GenericArray.html\" title=\"struct generic_array::GenericArray\">GenericArray</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    N: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;T&gt;,</span>"]],
"rhai":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"smartstring/struct.SmartString.html\" title=\"struct smartstring::SmartString\">SmartString</a>&lt;<a class=\"struct\" href=\"smartstring/config/struct.LazyCompact.html\" title=\"struct smartstring::config::LazyCompact\">LazyCompact</a>&gt;&gt; for <a class=\"struct\" href=\"rhai/plugin/struct.ImmutableString.html\" title=\"struct rhai::plugin::ImmutableString\">ImmutableString</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"rhai/plugin/struct.Module.html\" title=\"struct rhai::plugin::Module\">Module</a>&gt; for <a class=\"struct\" href=\"rhai/struct.AST.html\" title=\"struct rhai::AST\">AST</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"rhai/plugin/struct.ImmutableString.html\" title=\"struct rhai::plugin::ImmutableString\">ImmutableString</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[Stmt]&gt; for <a class=\"struct\" href=\"rhai/struct.AST.html\" title=\"struct rhai::AST\">AST</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"rhai/struct.Shared.html\" title=\"struct rhai::Shared\">Rc</a>&lt;<a class=\"struct\" href=\"rhai/plugin/struct.Module.html\" title=\"struct rhai::plugin::Module\">Module</a>&gt;&gt; for <a class=\"struct\" href=\"rhai/struct.AST.html\" title=\"struct rhai::AST\">AST</a>"]],
"serde_bytes":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"serde_bytes/struct.Bytes.html\" title=\"struct serde_bytes::Bytes\">Bytes</a>&gt; for <a class=\"struct\" href=\"serde_bytes/struct.ByteBuf.html\" title=\"struct serde_bytes::ByteBuf\">ByteBuf</a>"]],
"smallvec":[["impl&lt;A: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[&lt;A as <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a>]&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;"]],
"smartstring":[["impl&lt;Mode: <a class=\"trait\" href=\"smartstring/trait.SmartStringMode.html\" title=\"trait smartstring::SmartStringMode\">SmartStringMode</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"smartstring/struct.SmartString.html\" title=\"struct smartstring::SmartString\">SmartString</a>&lt;Mode&gt;"]],
"toml_edit":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"toml_edit/struct.InternalString.html\" title=\"struct toml_edit::InternalString\">InternalString</a>"]],
"winnow":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"winnow/stream/struct.BStr.html\" title=\"struct winnow::stream::BStr\">BStr</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"winnow/stream/struct.Bytes.html\" title=\"struct winnow::stream::Bytes\">Bytes</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.u8.html\">u8</a>&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()