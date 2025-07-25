use {
    super::{Ctx, MyF32, MyF64, Tester, SEED},
    anyhow::Result,
    once_cell::sync::Lazy,
    proptest::strategy::{Just, Strategy},
    wasmtime::{
        component::{HasSelf, InstancePre, Linker},
        Store,
    },
};

wasmtime::component::bindgen!({
    path: "src/test/wit",
    world: "echoes-test",
    async: true,
    trappable_imports: true,
});

impl componentize_py::test::echoes::Host for Ctx {
    async fn echo_nothing(&mut self) -> Result<()> {
        Ok(())
    }

    async fn echo_bool(&mut self, v: bool) -> Result<bool> {
        Ok(v)
    }

    async fn echo_u8(&mut self, v: u8) -> Result<u8> {
        Ok(v)
    }

    async fn echo_s8(&mut self, v: i8) -> Result<i8> {
        Ok(v)
    }

    async fn echo_u16(&mut self, v: u16) -> Result<u16> {
        Ok(v)
    }

    async fn echo_s16(&mut self, v: i16) -> Result<i16> {
        Ok(v)
    }

    async fn echo_u32(&mut self, v: u32) -> Result<u32> {
        Ok(v)
    }

    async fn echo_s32(&mut self, v: i32) -> Result<i32> {
        Ok(v)
    }

    async fn echo_char(&mut self, v: char) -> Result<char> {
        Ok(v)
    }

    async fn echo_u64(&mut self, v: u64) -> Result<u64> {
        Ok(v)
    }

    async fn echo_s64(&mut self, v: i64) -> Result<i64> {
        Ok(v)
    }

    async fn echo_f32(&mut self, v: f32) -> Result<f32> {
        Ok(v)
    }

    async fn echo_f64(&mut self, v: f64) -> Result<f64> {
        Ok(v)
    }

    async fn echo_string(&mut self, v: String) -> Result<String> {
        Ok(v)
    }

    async fn echo_list_bool(&mut self, v: Vec<bool>) -> Result<Vec<bool>> {
        Ok(v)
    }

    async fn echo_list_u8(&mut self, v: Vec<u8>) -> Result<Vec<u8>> {
        Ok(v)
    }

    async fn echo_list_s8(&mut self, v: Vec<i8>) -> Result<Vec<i8>> {
        Ok(v)
    }

    async fn echo_list_u16(&mut self, v: Vec<u16>) -> Result<Vec<u16>> {
        Ok(v)
    }

    async fn echo_list_s16(&mut self, v: Vec<i16>) -> Result<Vec<i16>> {
        Ok(v)
    }

    async fn echo_list_u32(&mut self, v: Vec<u32>) -> Result<Vec<u32>> {
        Ok(v)
    }

    async fn echo_list_s32(&mut self, v: Vec<i32>) -> Result<Vec<i32>> {
        Ok(v)
    }

    async fn echo_list_char(&mut self, v: Vec<char>) -> Result<Vec<char>> {
        Ok(v)
    }

    async fn echo_list_u64(&mut self, v: Vec<u64>) -> Result<Vec<u64>> {
        Ok(v)
    }

    async fn echo_list_s64(&mut self, v: Vec<i64>) -> Result<Vec<i64>> {
        Ok(v)
    }

    async fn echo_list_f32(&mut self, v: Vec<f32>) -> Result<Vec<f32>> {
        Ok(v)
    }

    async fn echo_list_f64(&mut self, v: Vec<f64>) -> Result<Vec<f64>> {
        Ok(v)
    }

    async fn echo_list_string(&mut self, v: Vec<String>) -> Result<Vec<String>> {
        Ok(v)
    }

    async fn echo_list_list_u8(&mut self, v: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>> {
        Ok(v)
    }

    async fn echo_list_list_list_u8(&mut self, v: Vec<Vec<Vec<u8>>>) -> Result<Vec<Vec<Vec<u8>>>> {
        Ok(v)
    }

    async fn echo_option_u8(&mut self, v: Option<u8>) -> Result<Option<u8>> {
        Ok(v)
    }

    async fn echo_option_option_u8(&mut self, v: Option<Option<u8>>) -> Result<Option<Option<u8>>> {
        Ok(v)
    }

    async fn echo_many(
        &mut self,
        v1: bool,
        v2: u8,
        v3: u16,
        v4: u32,
        v5: u64,
        v6: i8,
        v7: i16,
        v8: i32,
        v9: i64,
        v10: f32,
        v11: f64,
        v12: char,
        v13: String,
        v14: Vec<bool>,
        v15: Vec<u8>,
        v16: Vec<u16>,
    ) -> Result<(
        bool,
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        char,
        String,
        Vec<bool>,
        Vec<u8>,
        Vec<u16>,
    )> {
        Ok((
            v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16,
        ))
    }
}

struct Host;

impl super::Host for Host {
    type World = EchoesTest;

    fn add_to_linker(linker: &mut Linker<Ctx>) -> Result<()> {
        wasmtime_wasi::p2::add_to_linker_async(&mut *linker)?;
        componentize_py::test::echoes::add_to_linker::<_, HasSelf<_>>(linker, |ctx| ctx)?;
        Ok(())
    }

    async fn instantiate_pre(store: &mut Store<Ctx>, pre: InstancePre<Ctx>) -> Result<Self::World> {
        EchoesTestPre::new(pre)?.instantiate_async(store).await
    }
}

const GUEST_CODE: &[(&str, &str)] = &[(
    "app.py",
    r#"
from echoes_test import exports
from echoes_test.imports import echoes

class Echoes(exports.Echoes):
    def echo_nothing(self):
        echoes.echo_nothing()

    def echo_bool(self, v):
        return echoes.echo_bool(v)

    def echo_u8(self, v):
        assert v >= 0
        return echoes.echo_u8(v)

    def echo_s8(self, v):
        return echoes.echo_s8(v)

    def echo_u16(self, v):
        assert v >= 0
        return echoes.echo_u16(v)

    def echo_s16(self, v):
        return echoes.echo_s16(v)

    def echo_u32(self, v):
        assert v >= 0
        return echoes.echo_u32(v)

    def echo_s32(self, v):
        return echoes.echo_s32(v)

    def echo_char(self, v):
        return echoes.echo_char(v)

    def echo_u64(self, v):
        assert v >= 0
        return echoes.echo_u64(v)

    def echo_s64(self, v):
        return echoes.echo_s64(v)

    def echo_f32(self, v):
        return echoes.echo_f32(v)

    def echo_f64(self, v):
        return echoes.echo_f64(v)

    def echo_string(self, v):
        return echoes.echo_string(v)

    def echo_list_bool(self, v):
        return echoes.echo_list_bool(v)

    def echo_list_u8(self, v):
        assert all([n >= 0 for n in v])
        return echoes.echo_list_u8(v)

    def echo_list_s8(self, v):
        return echoes.echo_list_s8(v)

    def echo_list_u16(self, v):
        assert all([n >= 0 for n in v])
        return echoes.echo_list_u16(v)

    def echo_list_s16(self, v):
        return echoes.echo_list_s16(v)

    def echo_list_u32(self, v):
        assert all([n >= 0 for n in v])
        return echoes.echo_list_u32(v)

    def echo_list_s32(self, v):
        return echoes.echo_list_s32(v)

    def echo_list_char(self, v):
        return echoes.echo_list_char(v)

    def echo_list_u64(self, v):
        assert all([n >= 0 for n in v])
        return echoes.echo_list_u64(v)

    def echo_list_s64(self, v):
        return echoes.echo_list_s64(v)

    def echo_list_f32(self, v):
        return echoes.echo_list_f32(v)

    def echo_list_f64(self, v):
        return echoes.echo_list_f64(v)

    def echo_list_string(self, v):
        return echoes.echo_list_string(v)

    def echo_list_list_u8(self, v):
        return echoes.echo_list_list_u8(v)

    def echo_list_list_list_u8(self, v):
        return echoes.echo_list_list_list_u8(v)

    def echo_option_u8(self, v):
        return echoes.echo_option_u8(v)

    def echo_option_option_u8(self, v):
        return echoes.echo_option_option_u8(v)

    def echo_many(self, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16):
        return echoes.echo_many(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
"#,
)];

static TESTER: Lazy<Tester<Host>> = Lazy::new(|| {
    Tester::<Host>::new(
        include_str!("wit/echoes.wit"),
        Some("echoes_test"),
        GUEST_CODE,
        &[],
        &[],
        *SEED,
    )
    .unwrap()
});

#[test]
fn nothing() -> Result<()> {
    TESTER.all_eq(&Just(()), |(), instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_nothing(store),
        )
    })
}

#[test]
fn bools() -> Result<()> {
    TESTER.all_eq(&proptest::bool::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_bool(store, v),
        )
    })
}

#[test]
fn u8s() -> Result<()> {
    TESTER.all_eq(&proptest::num::u8::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_u8(store, v),
        )
    })
}

#[test]
fn s8s() -> Result<()> {
    TESTER.all_eq(&proptest::num::i8::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_s8(store, v),
        )
    })
}

#[test]
fn u16s() -> Result<()> {
    TESTER.all_eq(&proptest::num::u16::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_u16(store, v),
        )
    })
}

#[test]
fn s16s() -> Result<()> {
    TESTER.all_eq(&proptest::num::i16::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_s16(store, v),
        )
    })
}

#[test]
fn u32s() -> Result<()> {
    TESTER.all_eq(&proptest::num::u32::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_u32(store, v),
        )
    })
}

#[test]
fn s32s() -> Result<()> {
    TESTER.all_eq(&proptest::num::i32::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_s32(store, v),
        )
    })
}

#[test]
fn u64s() -> Result<()> {
    TESTER.all_eq(&proptest::num::u64::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_u64(store, v),
        )
    })
}

#[test]
fn s64s() -> Result<()> {
    TESTER.all_eq(&proptest::num::i64::ANY, |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_s64(store, v),
        )
    })
}

#[test]
fn chars() -> Result<()> {
    TESTER.all_eq(&proptest::char::any(), |v, instance, store, runtime| {
        runtime.block_on(
            instance
                .componentize_py_test_echoes()
                .call_echo_char(store, v),
        )
    })
}

#[test]
fn f32s() -> Result<()> {
    TESTER.all_eq(
        &proptest::num::f32::ANY.prop_map(MyF32),
        |v, instance, store, runtime| {
            Ok(MyF32(
                runtime.block_on(
                    instance
                        .componentize_py_test_echoes()
                        .call_echo_f32(store, v.0),
                )?,
            ))
        },
    )
}

#[test]
fn f64s() -> Result<()> {
    TESTER.all_eq(
        &proptest::num::f64::ANY.prop_map(MyF64),
        |v, instance, store, runtime| {
            Ok(MyF64(
                runtime.block_on(
                    instance
                        .componentize_py_test_echoes()
                        .call_echo_f64(store, v.0),
                )?,
            ))
        },
    )
}

const MAX_SIZE: usize = 100;

#[test]
fn strings() -> Result<()> {
    TESTER.all_eq(
        &proptest::string::string_regex(".*")?,
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_string(store, &v),
            )
        },
    )
}

#[test]
fn list_bools() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::bool::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_bool(store, &v),
            )
        },
    )
}

#[test]
fn list_u8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::u8::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_u8(store, &v),
            )
        },
    )
}

#[test]
fn list_list_u8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(
            proptest::collection::vec(proptest::num::u8::ANY, 0..MAX_SIZE / 2),
            0..MAX_SIZE,
        ),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_list_u8(store, &v),
            )
        },
    )
}

#[test]
fn list_list_list_u8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(
            proptest::collection::vec(
                proptest::collection::vec(proptest::num::u8::ANY, 0..MAX_SIZE / 4),
                0..MAX_SIZE / 2,
            ),
            0..MAX_SIZE,
        ),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_list_list_u8(store, &v),
            )
        },
    )
}

#[test]
fn option_u8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::option::of(proptest::num::u8::ANY),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_option_u8(store, v),
            )
        },
    )
}

#[test]
fn option_option_u8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::option::of(proptest::option::of(proptest::num::u8::ANY)),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_option_option_u8(store, v),
            )
        },
    )
}

#[test]
fn list_s8s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::i8::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_s8(store, &v),
            )
        },
    )
}

#[test]
fn list_u16s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::u16::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_u16(store, &v),
            )
        },
    )
}

#[test]
fn list_s16s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::i16::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_s16(store, &v),
            )
        },
    )
}

#[test]
fn list_u32s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::u32::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_u32(store, &v),
            )
        },
    )
}

#[test]
fn list_s32s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::i32::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_s32(store, &v),
            )
        },
    )
}

#[test]
fn list_u64s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::u64::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_u64(store, &v),
            )
        },
    )
}

#[test]
fn list_s64s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::i64::ANY, 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_s64(store, &v),
            )
        },
    )
}

#[test]
fn list_chars() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::char::any(), 0..MAX_SIZE),
        |v, instance, store, runtime| {
            runtime.block_on(
                instance
                    .componentize_py_test_echoes()
                    .call_echo_list_char(store, &v),
            )
        },
    )
}

#[test]
fn list_f32s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::f32::ANY.prop_map(MyF32), 0..MAX_SIZE),
        |v, instance, store, runtime| {
            Ok(runtime
                .block_on(
                    instance
                        .componentize_py_test_echoes()
                        .call_echo_list_f32(store, &v.into_iter().map(|v| v.0).collect::<Vec<_>>()),
                )?
                .into_iter()
                .map(MyF32)
                .collect())
        },
    )
}

#[test]
fn list_f64s() -> Result<()> {
    TESTER.all_eq(
        &proptest::collection::vec(proptest::num::f64::ANY.prop_map(MyF64), 0..MAX_SIZE),
        |v, instance, store, runtime| {
            Ok(runtime
                .block_on(
                    instance
                        .componentize_py_test_echoes()
                        .call_echo_list_f64(store, &v.into_iter().map(|v| v.0).collect::<Vec<_>>()),
                )?
                .into_iter()
                .map(MyF64)
                .collect())
        },
    )
}

#[test]
fn many() -> Result<()> {
    TESTER.all_eq(
        &(
            (
                proptest::bool::ANY,
                proptest::num::u8::ANY,
                proptest::num::u16::ANY,
                proptest::num::u32::ANY,
                proptest::num::u64::ANY,
                proptest::num::i8::ANY,
                proptest::num::i16::ANY,
                proptest::num::i32::ANY,
            ),
            (
                proptest::num::i64::ANY,
                proptest::num::f32::ANY.prop_map(MyF32),
                proptest::num::f64::ANY.prop_map(MyF64),
                proptest::char::any(),
                proptest::string::string_regex(".*")?,
                proptest::collection::vec(proptest::bool::ANY, 0..MAX_SIZE),
                proptest::collection::vec(proptest::num::u8::ANY, 0..MAX_SIZE),
                proptest::collection::vec(proptest::num::u16::ANY, 0..MAX_SIZE),
            ),
        ),
        |((v1, v2, v3, v4, v5, v6, v7, v8), (v9, v10, v11, v12, v13, v14, v15, v16)),
         instance,
         store,
         runtime| {
            let (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) =
                runtime.block_on(instance.componentize_py_test_echoes().call_echo_many(
                    store, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10.0, v11.0, v12, &v13, &v14, &v15,
                    &v16,
                ))?;

            Ok((
                (v1, v2, v3, v4, v5, v6, v7, v8),
                (v9, MyF32(v10), MyF64(v11), v12, v13, v14, v15, v16),
            ))
        },
    )
}
