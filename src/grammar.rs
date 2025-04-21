// auto-generated: "lalrpop 0.22.1"
// sha3: a02f336cfcd540ec2c3088863eaea938af6028af4ff992f0dd206f909afc78fb
use crate::tokens::{Token, LexicalError};
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Command {

    use crate::tokens::{Token, LexicalError};
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(i32),
        Variant1((u32, u32)),
        Variant2((String, u32, u32)),
        Variant3(Token),
        Variant4(Addr),
        Variant5((Box<Expr>, Vec<ParentType>)),
        Variant6((Command, Vec<ParentType>)),
        Variant7((DisplayCommand, Vec<ParentType>)),
        Variant8(BinaryFunction),
        Variant9(MonoFunction),
        Variant10(RangeFunction),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 26, 25, 22, 20, 23, 21, 24, 0, 0, 0, 0, 0, 0, 19, 18, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 26, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        39, 26, 25, 0, 0, 0, 0, 0, 38, 33, 35, 34, 37, 36, 0, 0, 0, 6, 0, 0, 0, 0, 32, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 41, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20, -20, 42, 43,
        // State 5
        39, 26, 25, 0, 0, 0, 0, 0, 38, 33, 35, 34, 37, 36, 0, 0, 0, 6, 0, 0, 0, 0, 32, 0, 0,
        // State 6
        39, 26, 25, 0, 0, 0, 0, 0, 38, 33, 35, 34, 37, 36, 0, 0, 0, 6, 0, 0, 0, 0, 32, 0, 0,
        // State 7
        39, 26, 25, 0, 0, 0, 0, 0, 38, 33, 35, 34, 37, 36, 0, 0, 0, 6, 0, 0, 0, 0, 32, 0, 0,
        // State 8
        39, 26, 25, 0, 0, 0, 0, 0, 38, 33, 35, 34, 37, 36, 0, 0, 0, 6, 0, 0, 0, 0, 32, 0, 0,
        // State 9
        0, 26, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 40, 41, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, -19, 42, 43,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 40, 41, 0, 0,
        // State 13
        0, 26, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -2, -2, -2, -2, -2, -2,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, -5, -5, -5, -5,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, -24, -24, -24, -24,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, -3, -3, -3, -3,
        // State 39
        -21, -21, -21, 0, 0, 0, 0, 0, -21, -21, -21, -21, -21, -21, 0, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0,
        // State 40
        -22, -22, -22, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, 0, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0,
        // State 41
        -25, -25, -25, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, 0, 0, 0, -25, 0, 0, 0, 0, -25, 0, 0,
        // State 42
        -26, -26, -26, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, 0, 0, 0, -26, 0, 0, 0, 0, -26, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, -4, -4, -4,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, -8, -8, -8,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, -7, -7, -7,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, -6, -6, -6,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        -10,
        // State 4
        -20,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -19,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        -33,
        // State 16
        -9,
        // State 17
        -13,
        // State 18
        -12,
        // State 19
        -16,
        // State 20
        -18,
        // State 21
        -15,
        // State 22
        -17,
        // State 23
        -11,
        // State 24
        -2,
        // State 25
        -1,
        // State 26
        -14,
        // State 27
        -5,
        // State 28
        -24,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -3,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        -4,
        // State 44
        -23,
        // State 45
        0,
        // State 46
        -8,
        // State 47
        -7,
        // State 48
        0,
        // State 49
        -6,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                0 => 14,
                1 => 26,
                9 => 45,
                13 => 48,
                _ => 27,
            },
            1 => match state {
                7 => 44,
                _ => 28,
            },
            2 => 15,
            3 => 16,
            4 => match state {
                5 => 10,
                8 => 12,
                _ => 3,
            },
            5 => 6,
            6 => match state {
                6 => 11,
                _ => 4,
            },
            7 => 7,
            8 => 29,
            9 => 30,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###""int""###,
        r###""localcell""###,
        r###""globalcell""###,
        r###""MoveUp""###,
        r###""MoveLeft""###,
        r###""MovoDown""###,
        r###""MoveRight""###,
        r###""Quit""###,
        r###""Sum""###,
        r###""Avg""###,
        r###""Min""###,
        r###""Max""###,
        r###""Stdev""###,
        r###""Sleep""###,
        r###""EnableOut""###,
        r###""DisableOut""###,
        r###""ScrollTo""###,
        r###""(""###,
        r###"")""###,
        r###""=""###,
        r###"":""###,
        r###""+""###,
        r###""-""###,
        r###""*""###,
        r###""/""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i8],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<>
    where 
    {
        curr_sheet: u32,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = (Command, Vec<ParentType>);
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.curr_sheet,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token::Integer(_) if true => Some(0),
            Token::LocalCell(_) if true => Some(1),
            Token::GlobalCell(_) if true => Some(2),
            Token::MoveUp if true => Some(3),
            Token::MoveLeft if true => Some(4),
            Token::MoveDown if true => Some(5),
            Token::MoveRight if true => Some(6),
            Token::Quit if true => Some(7),
            Token::Sum if true => Some(8),
            Token::Avg if true => Some(9),
            Token::Min if true => Some(10),
            Token::Max if true => Some(11),
            Token::Stdev if true => Some(12),
            Token::Sleep if true => Some(13),
            Token::EnableOut if true => Some(14),
            Token::DisableOut if true => Some(15),
            Token::ScrollTo if true => Some(16),
            Token::LParen if true => Some(17),
            Token::RParen if true => Some(18),
            Token::Assign if true => Some(19),
            Token::Colon if true => Some(20),
            Token::OperatorAdd if true => Some(21),
            Token::OperatorSub if true => Some(22),
            Token::OperatorMul if true => Some(23),
            Token::OperatorDiv if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 => match __token {
                Token::Integer(__tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            1 => match __token {
                Token::LocalCell(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            2 => match __token {
                Token::GlobalCell(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => __Symbol::Variant3(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 1,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 1,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 1,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            32 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct CommandParser {
        _priv: (),
    }

    impl Default for CommandParser { fn default() -> Self { Self::new() } }
    impl CommandParser {
        pub fn new() -> CommandParser {
            CommandParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            curr_sheet: u32,
            __tokens0: __TOKENS,
        ) -> Result<(Command, Vec<ParentType>), __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    curr_sheet,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
    >(
        curr_sheet: u32,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<(Command, Vec<ParentType>),__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(curr_sheet, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                // __Command = Command => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(curr_sheet, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Box<Expr>, Vec<ParentType>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Command, Vec<ParentType>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (DisplayCommand, Vec<ParentType>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (String, u32, u32), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (u32, u32), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Addr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, BinaryFunction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, MonoFunction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, RangeFunction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Addr = "localcell" => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 0)
    }
    fn __reduce1<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Addr = "globalcell" => ActionFn(19);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 0)
    }
    fn __reduce2<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = "int" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = "-", "int" => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(curr_sheet, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 1)
    }
    fn __reduce4<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = Addr => ActionFn(14);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 1)
    }
    fn __reduce5<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = RangeFuncs, "(", Addr, ":", Addr, ")" => ActionFn(15);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action15::<>(curr_sheet, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (6, 1)
    }
    fn __reduce6<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = MonoFuncs, "(", Expr, ")" => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action16::<>(curr_sheet, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 1)
    }
    fn __reduce7<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Atom = "(", Expr, ")" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(curr_sheet, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 1)
    }
    fn __reduce8<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Command = DisplayCommand => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 2)
    }
    fn __reduce9<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Command = Addr, "=", Expr => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action2::<>(curr_sheet, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 2)
    }
    fn __reduce10<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Command = "Quit" => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 2)
    }
    fn __reduce11<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "EnableOut" => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce12<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "DisableOut" => ActionFn(27);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce13<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "ScrollTo", Addr => ActionFn(28);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action28::<>(curr_sheet, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 3)
    }
    fn __reduce14<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "MoveUp" => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce15<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "MoveLeft" => ActionFn(30);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce16<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "MovoDown" => ActionFn(31);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce17<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DisplayCommand = "MoveRight" => ActionFn(32);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 3)
    }
    fn __reduce18<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action4::<>(curr_sheet, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 4)
    }
    fn __reduce19<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(5);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    fn __reduce20<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 5)
    }
    fn __reduce21<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 5)
    }
    fn __reduce22<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Atom => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action8::<>(curr_sheet, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 6)
    }
    fn __reduce23<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Atom => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
    }
    fn __reduce24<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(10);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action10::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 7)
    }
    fn __reduce25<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(11);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 7)
    }
    fn __reduce26<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MonoFuncs = "Sleep" => ActionFn(25);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 8)
    }
    fn __reduce27<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // RangeFuncs = "Sum" => ActionFn(20);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action20::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    fn __reduce28<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // RangeFuncs = "Avg" => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    fn __reduce29<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // RangeFuncs = "Min" => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    fn __reduce30<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // RangeFuncs = "Max" => ActionFn(23);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    fn __reduce31<
    >(
        curr_sheet: u32,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // RangeFuncs = "Stdev" => ActionFn(24);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(curr_sheet, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Command::CommandParser;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, (Command, Vec<ParentType>), usize),
) -> (Command, Vec<ParentType>)
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
>(
    curr_sheet: u32,
    (_, c_v, _): (usize, (DisplayCommand, Vec<ParentType>), usize),
) -> (Command, Vec<ParentType>)
{
    {
        let (c, v) = c_v;
        (Command::DisplayCmd(c), v)}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
>(
    curr_sheet: u32,
    (_, a, _): (usize, Addr, usize),
    (_, _, _): (usize, Token, usize),
    (_, e_v, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
) -> (Command, Vec<ParentType>)
{
    {
        let (e, v) = e_v;
        (Command::AssignCmd(a, e), v)}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (Command, Vec<ParentType>)
{
    (Command::Quit, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
>(
    curr_sheet: u32,
    (_, e_v1, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
    (_, o, _): (usize, BinaryFunction, usize),
    (_, f_v2, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    { 
        let ((e, mut v1), (f, mut v2)) = (e_v1, f_v2);
        v1.append(&mut v2);
        (Box::new(Expr::BinOp(e, o, f)), v1)}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> BinaryFunction
{
    BinaryFunction::Add
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> BinaryFunction
{
    BinaryFunction::Sub
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
>(
    curr_sheet: u32,
    (_, f_v1, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
    (_, o, _): (usize, BinaryFunction, usize),
    (_, a_v2, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    { 
    let ((f, mut v1), (a, mut v2)) = (f_v1, a_v2);
    v1.append(&mut v2);
    (Box::new(Expr::BinOp(f, o, a)), v1)}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> BinaryFunction
{
    BinaryFunction::Mul
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> BinaryFunction
{
    BinaryFunction::Div
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, i32, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    (Box::new(Expr::Integer(__0)), vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
>(
    curr_sheet: u32,
    (_, _, _): (usize, Token, usize),
    (_, i, _): (usize, i32, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    (Box::new(Expr::Integer(-i)), vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
>(
    curr_sheet: u32,
    (_, a, _): (usize, Addr, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    (Box::new(Expr::Cell(a.clone())), vec![ParentType::Single(a)])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
>(
    curr_sheet: u32,
    (_, f, _): (usize, RangeFunction, usize),
    (_, _, _): (usize, Token, usize),
    (_, a1, _): (usize, Addr, usize),
    (_, _, _): (usize, Token, usize),
    (_, a2, _): (usize, Addr, usize),
    (_, _, _): (usize, Token, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    (Box::new(Expr::RangeOp{op: f, start: a1.clone(), end: a2.clone()}), vec![ParentType::Range(a1, a2)])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
>(
    curr_sheet: u32,
    (_, f, _): (usize, MonoFunction, usize),
    (_, _, _): (usize, Token, usize),
    (_, e_v, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
    (_, _, _): (usize, Token, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    {let (e, v) = e_v; (Box::new(Expr::MonoOp(f, e)), v)}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
>(
    curr_sheet: u32,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, (Box<Expr>, Vec<ParentType>), usize),
    (_, _, _): (usize, Token, usize),
) -> (Box<Expr>, Vec<ParentType>)
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
>(
    curr_sheet: u32,
    (_, cr, _): (usize, (u32, u32), usize),
) -> Addr
{
    { 
        let (col, row) = cr;
        Addr{sheet: curr_sheet, col, row} }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
>(
    curr_sheet: u32,
    (_, scr, _): (usize, (String, u32, u32), usize),
) -> Addr
{
    { 
        let (sheet, col, row) = scr;
        Addr{sheet: 99, col, row} }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> RangeFunction
{
    RangeFunction::Sum
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> RangeFunction
{
    RangeFunction::Avg
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> RangeFunction
{
    RangeFunction::Min
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> RangeFunction
{
    RangeFunction::Max
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> RangeFunction
{
    RangeFunction::Stdev
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> MonoFunction
{
    MonoFunction::Sleep
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::EnableOut, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::DisableOut, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
>(
    curr_sheet: u32,
    (_, _, _): (usize, Token, usize),
    (_, a, _): (usize, Addr, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::ScrollTo(a.clone()), vec![ParentType::Single(a)])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::MoveUp, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::MoveLeft, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::MoveDown, vec![])
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
>(
    curr_sheet: u32,
    (_, __0, _): (usize, Token, usize),
) -> (DisplayCommand, Vec<ParentType>)
{
    (DisplayCommand::MoveRight, vec![])
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple<>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>>;
}

impl<> __ToTriple<> for (usize, Token, usize)
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        Ok(self)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), LexicalError>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        self.map_err(|error| __lalrpop_util::ParseError::User { error })
    }
}
