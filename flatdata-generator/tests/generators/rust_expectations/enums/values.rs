#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum EnumI8 {
    FooI8Neg = -128,
    FooI8Pos = 127,
    FooI8Zero = 0,
    FooI8NegHex = -127,
    FooI8PosHex = 126,
    FooI8OneHex = 1,
    #[doc(hidden)]
    UnknownValueMinus126 = -126,
    #[doc(hidden)]
    UnknownValueMinus125 = -125,
    #[doc(hidden)]
    UnknownValueMinus124 = -124,
    #[doc(hidden)]
    UnknownValueMinus123 = -123,
    #[doc(hidden)]
    UnknownValueMinus122 = -122,
    #[doc(hidden)]
    UnknownValueMinus121 = -121,
    #[doc(hidden)]
    UnknownValueMinus120 = -120,
    #[doc(hidden)]
    UnknownValueMinus119 = -119,
    #[doc(hidden)]
    UnknownValueMinus118 = -118,
    #[doc(hidden)]
    UnknownValueMinus117 = -117,
    #[doc(hidden)]
    UnknownValueMinus116 = -116,
    #[doc(hidden)]
    UnknownValueMinus115 = -115,
    #[doc(hidden)]
    UnknownValueMinus114 = -114,
    #[doc(hidden)]
    UnknownValueMinus113 = -113,
    #[doc(hidden)]
    UnknownValueMinus112 = -112,
    #[doc(hidden)]
    UnknownValueMinus111 = -111,
    #[doc(hidden)]
    UnknownValueMinus110 = -110,
    #[doc(hidden)]
    UnknownValueMinus109 = -109,
    #[doc(hidden)]
    UnknownValueMinus108 = -108,
    #[doc(hidden)]
    UnknownValueMinus107 = -107,
    #[doc(hidden)]
    UnknownValueMinus106 = -106,
    #[doc(hidden)]
    UnknownValueMinus105 = -105,
    #[doc(hidden)]
    UnknownValueMinus104 = -104,
    #[doc(hidden)]
    UnknownValueMinus103 = -103,
    #[doc(hidden)]
    UnknownValueMinus102 = -102,
    #[doc(hidden)]
    UnknownValueMinus101 = -101,
    #[doc(hidden)]
    UnknownValueMinus100 = -100,
    #[doc(hidden)]
    UnknownValueMinus99 = -99,
    #[doc(hidden)]
    UnknownValueMinus98 = -98,
    #[doc(hidden)]
    UnknownValueMinus97 = -97,
    #[doc(hidden)]
    UnknownValueMinus96 = -96,
    #[doc(hidden)]
    UnknownValueMinus95 = -95,
    #[doc(hidden)]
    UnknownValueMinus94 = -94,
    #[doc(hidden)]
    UnknownValueMinus93 = -93,
    #[doc(hidden)]
    UnknownValueMinus92 = -92,
    #[doc(hidden)]
    UnknownValueMinus91 = -91,
    #[doc(hidden)]
    UnknownValueMinus90 = -90,
    #[doc(hidden)]
    UnknownValueMinus89 = -89,
    #[doc(hidden)]
    UnknownValueMinus88 = -88,
    #[doc(hidden)]
    UnknownValueMinus87 = -87,
    #[doc(hidden)]
    UnknownValueMinus86 = -86,
    #[doc(hidden)]
    UnknownValueMinus85 = -85,
    #[doc(hidden)]
    UnknownValueMinus84 = -84,
    #[doc(hidden)]
    UnknownValueMinus83 = -83,
    #[doc(hidden)]
    UnknownValueMinus82 = -82,
    #[doc(hidden)]
    UnknownValueMinus81 = -81,
    #[doc(hidden)]
    UnknownValueMinus80 = -80,
    #[doc(hidden)]
    UnknownValueMinus79 = -79,
    #[doc(hidden)]
    UnknownValueMinus78 = -78,
    #[doc(hidden)]
    UnknownValueMinus77 = -77,
    #[doc(hidden)]
    UnknownValueMinus76 = -76,
    #[doc(hidden)]
    UnknownValueMinus75 = -75,
    #[doc(hidden)]
    UnknownValueMinus74 = -74,
    #[doc(hidden)]
    UnknownValueMinus73 = -73,
    #[doc(hidden)]
    UnknownValueMinus72 = -72,
    #[doc(hidden)]
    UnknownValueMinus71 = -71,
    #[doc(hidden)]
    UnknownValueMinus70 = -70,
    #[doc(hidden)]
    UnknownValueMinus69 = -69,
    #[doc(hidden)]
    UnknownValueMinus68 = -68,
    #[doc(hidden)]
    UnknownValueMinus67 = -67,
    #[doc(hidden)]
    UnknownValueMinus66 = -66,
    #[doc(hidden)]
    UnknownValueMinus65 = -65,
    #[doc(hidden)]
    UnknownValueMinus64 = -64,
    #[doc(hidden)]
    UnknownValueMinus63 = -63,
    #[doc(hidden)]
    UnknownValueMinus62 = -62,
    #[doc(hidden)]
    UnknownValueMinus61 = -61,
    #[doc(hidden)]
    UnknownValueMinus60 = -60,
    #[doc(hidden)]
    UnknownValueMinus59 = -59,
    #[doc(hidden)]
    UnknownValueMinus58 = -58,
    #[doc(hidden)]
    UnknownValueMinus57 = -57,
    #[doc(hidden)]
    UnknownValueMinus56 = -56,
    #[doc(hidden)]
    UnknownValueMinus55 = -55,
    #[doc(hidden)]
    UnknownValueMinus54 = -54,
    #[doc(hidden)]
    UnknownValueMinus53 = -53,
    #[doc(hidden)]
    UnknownValueMinus52 = -52,
    #[doc(hidden)]
    UnknownValueMinus51 = -51,
    #[doc(hidden)]
    UnknownValueMinus50 = -50,
    #[doc(hidden)]
    UnknownValueMinus49 = -49,
    #[doc(hidden)]
    UnknownValueMinus48 = -48,
    #[doc(hidden)]
    UnknownValueMinus47 = -47,
    #[doc(hidden)]
    UnknownValueMinus46 = -46,
    #[doc(hidden)]
    UnknownValueMinus45 = -45,
    #[doc(hidden)]
    UnknownValueMinus44 = -44,
    #[doc(hidden)]
    UnknownValueMinus43 = -43,
    #[doc(hidden)]
    UnknownValueMinus42 = -42,
    #[doc(hidden)]
    UnknownValueMinus41 = -41,
    #[doc(hidden)]
    UnknownValueMinus40 = -40,
    #[doc(hidden)]
    UnknownValueMinus39 = -39,
    #[doc(hidden)]
    UnknownValueMinus38 = -38,
    #[doc(hidden)]
    UnknownValueMinus37 = -37,
    #[doc(hidden)]
    UnknownValueMinus36 = -36,
    #[doc(hidden)]
    UnknownValueMinus35 = -35,
    #[doc(hidden)]
    UnknownValueMinus34 = -34,
    #[doc(hidden)]
    UnknownValueMinus33 = -33,
    #[doc(hidden)]
    UnknownValueMinus32 = -32,
    #[doc(hidden)]
    UnknownValueMinus31 = -31,
    #[doc(hidden)]
    UnknownValueMinus30 = -30,
    #[doc(hidden)]
    UnknownValueMinus29 = -29,
    #[doc(hidden)]
    UnknownValueMinus28 = -28,
    #[doc(hidden)]
    UnknownValueMinus27 = -27,
    #[doc(hidden)]
    UnknownValueMinus26 = -26,
    #[doc(hidden)]
    UnknownValueMinus25 = -25,
    #[doc(hidden)]
    UnknownValueMinus24 = -24,
    #[doc(hidden)]
    UnknownValueMinus23 = -23,
    #[doc(hidden)]
    UnknownValueMinus22 = -22,
    #[doc(hidden)]
    UnknownValueMinus21 = -21,
    #[doc(hidden)]
    UnknownValueMinus20 = -20,
    #[doc(hidden)]
    UnknownValueMinus19 = -19,
    #[doc(hidden)]
    UnknownValueMinus18 = -18,
    #[doc(hidden)]
    UnknownValueMinus17 = -17,
    #[doc(hidden)]
    UnknownValueMinus16 = -16,
    #[doc(hidden)]
    UnknownValueMinus15 = -15,
    #[doc(hidden)]
    UnknownValueMinus14 = -14,
    #[doc(hidden)]
    UnknownValueMinus13 = -13,
    #[doc(hidden)]
    UnknownValueMinus12 = -12,
    #[doc(hidden)]
    UnknownValueMinus11 = -11,
    #[doc(hidden)]
    UnknownValueMinus10 = -10,
    #[doc(hidden)]
    UnknownValueMinus9 = -9,
    #[doc(hidden)]
    UnknownValueMinus8 = -8,
    #[doc(hidden)]
    UnknownValueMinus7 = -7,
    #[doc(hidden)]
    UnknownValueMinus6 = -6,
    #[doc(hidden)]
    UnknownValueMinus5 = -5,
    #[doc(hidden)]
    UnknownValueMinus4 = -4,
    #[doc(hidden)]
    UnknownValueMinus3 = -3,
    #[doc(hidden)]
    UnknownValueMinus2 = -2,
    #[doc(hidden)]
    UnknownValueMinus1 = -1,
    #[doc(hidden)]
    UnknownValue2 = 2,
    #[doc(hidden)]
    UnknownValue3 = 3,
    #[doc(hidden)]
    UnknownValue4 = 4,
    #[doc(hidden)]
    UnknownValue5 = 5,
    #[doc(hidden)]
    UnknownValue6 = 6,
    #[doc(hidden)]
    UnknownValue7 = 7,
    #[doc(hidden)]
    UnknownValue8 = 8,
    #[doc(hidden)]
    UnknownValue9 = 9,
    #[doc(hidden)]
    UnknownValue10 = 10,
    #[doc(hidden)]
    UnknownValue11 = 11,
    #[doc(hidden)]
    UnknownValue12 = 12,
    #[doc(hidden)]
    UnknownValue13 = 13,
    #[doc(hidden)]
    UnknownValue14 = 14,
    #[doc(hidden)]
    UnknownValue15 = 15,
    #[doc(hidden)]
    UnknownValue16 = 16,
    #[doc(hidden)]
    UnknownValue17 = 17,
    #[doc(hidden)]
    UnknownValue18 = 18,
    #[doc(hidden)]
    UnknownValue19 = 19,
    #[doc(hidden)]
    UnknownValue20 = 20,
    #[doc(hidden)]
    UnknownValue21 = 21,
    #[doc(hidden)]
    UnknownValue22 = 22,
    #[doc(hidden)]
    UnknownValue23 = 23,
    #[doc(hidden)]
    UnknownValue24 = 24,
    #[doc(hidden)]
    UnknownValue25 = 25,
    #[doc(hidden)]
    UnknownValue26 = 26,
    #[doc(hidden)]
    UnknownValue27 = 27,
    #[doc(hidden)]
    UnknownValue28 = 28,
    #[doc(hidden)]
    UnknownValue29 = 29,
    #[doc(hidden)]
    UnknownValue30 = 30,
    #[doc(hidden)]
    UnknownValue31 = 31,
    #[doc(hidden)]
    UnknownValue32 = 32,
    #[doc(hidden)]
    UnknownValue33 = 33,
    #[doc(hidden)]
    UnknownValue34 = 34,
    #[doc(hidden)]
    UnknownValue35 = 35,
    #[doc(hidden)]
    UnknownValue36 = 36,
    #[doc(hidden)]
    UnknownValue37 = 37,
    #[doc(hidden)]
    UnknownValue38 = 38,
    #[doc(hidden)]
    UnknownValue39 = 39,
    #[doc(hidden)]
    UnknownValue40 = 40,
    #[doc(hidden)]
    UnknownValue41 = 41,
    #[doc(hidden)]
    UnknownValue42 = 42,
    #[doc(hidden)]
    UnknownValue43 = 43,
    #[doc(hidden)]
    UnknownValue44 = 44,
    #[doc(hidden)]
    UnknownValue45 = 45,
    #[doc(hidden)]
    UnknownValue46 = 46,
    #[doc(hidden)]
    UnknownValue47 = 47,
    #[doc(hidden)]
    UnknownValue48 = 48,
    #[doc(hidden)]
    UnknownValue49 = 49,
    #[doc(hidden)]
    UnknownValue50 = 50,
    #[doc(hidden)]
    UnknownValue51 = 51,
    #[doc(hidden)]
    UnknownValue52 = 52,
    #[doc(hidden)]
    UnknownValue53 = 53,
    #[doc(hidden)]
    UnknownValue54 = 54,
    #[doc(hidden)]
    UnknownValue55 = 55,
    #[doc(hidden)]
    UnknownValue56 = 56,
    #[doc(hidden)]
    UnknownValue57 = 57,
    #[doc(hidden)]
    UnknownValue58 = 58,
    #[doc(hidden)]
    UnknownValue59 = 59,
    #[doc(hidden)]
    UnknownValue60 = 60,
    #[doc(hidden)]
    UnknownValue61 = 61,
    #[doc(hidden)]
    UnknownValue62 = 62,
    #[doc(hidden)]
    UnknownValue63 = 63,
    #[doc(hidden)]
    UnknownValue64 = 64,
    #[doc(hidden)]
    UnknownValue65 = 65,
    #[doc(hidden)]
    UnknownValue66 = 66,
    #[doc(hidden)]
    UnknownValue67 = 67,
    #[doc(hidden)]
    UnknownValue68 = 68,
    #[doc(hidden)]
    UnknownValue69 = 69,
    #[doc(hidden)]
    UnknownValue70 = 70,
    #[doc(hidden)]
    UnknownValue71 = 71,
    #[doc(hidden)]
    UnknownValue72 = 72,
    #[doc(hidden)]
    UnknownValue73 = 73,
    #[doc(hidden)]
    UnknownValue74 = 74,
    #[doc(hidden)]
    UnknownValue75 = 75,
    #[doc(hidden)]
    UnknownValue76 = 76,
    #[doc(hidden)]
    UnknownValue77 = 77,
    #[doc(hidden)]
    UnknownValue78 = 78,
    #[doc(hidden)]
    UnknownValue79 = 79,
    #[doc(hidden)]
    UnknownValue80 = 80,
    #[doc(hidden)]
    UnknownValue81 = 81,
    #[doc(hidden)]
    UnknownValue82 = 82,
    #[doc(hidden)]
    UnknownValue83 = 83,
    #[doc(hidden)]
    UnknownValue84 = 84,
    #[doc(hidden)]
    UnknownValue85 = 85,
    #[doc(hidden)]
    UnknownValue86 = 86,
    #[doc(hidden)]
    UnknownValue87 = 87,
    #[doc(hidden)]
    UnknownValue88 = 88,
    #[doc(hidden)]
    UnknownValue89 = 89,
    #[doc(hidden)]
    UnknownValue90 = 90,
    #[doc(hidden)]
    UnknownValue91 = 91,
    #[doc(hidden)]
    UnknownValue92 = 92,
    #[doc(hidden)]
    UnknownValue93 = 93,
    #[doc(hidden)]
    UnknownValue94 = 94,
    #[doc(hidden)]
    UnknownValue95 = 95,
    #[doc(hidden)]
    UnknownValue96 = 96,
    #[doc(hidden)]
    UnknownValue97 = 97,
    #[doc(hidden)]
    UnknownValue98 = 98,
    #[doc(hidden)]
    UnknownValue99 = 99,
    #[doc(hidden)]
    UnknownValue100 = 100,
    #[doc(hidden)]
    UnknownValue101 = 101,
    #[doc(hidden)]
    UnknownValue102 = 102,
    #[doc(hidden)]
    UnknownValue103 = 103,
    #[doc(hidden)]
    UnknownValue104 = 104,
    #[doc(hidden)]
    UnknownValue105 = 105,
    #[doc(hidden)]
    UnknownValue106 = 106,
    #[doc(hidden)]
    UnknownValue107 = 107,
    #[doc(hidden)]
    UnknownValue108 = 108,
    #[doc(hidden)]
    UnknownValue109 = 109,
    #[doc(hidden)]
    UnknownValue110 = 110,
    #[doc(hidden)]
    UnknownValue111 = 111,
    #[doc(hidden)]
    UnknownValue112 = 112,
    #[doc(hidden)]
    UnknownValue113 = 113,
    #[doc(hidden)]
    UnknownValue114 = 114,
    #[doc(hidden)]
    UnknownValue115 = 115,
    #[doc(hidden)]
    UnknownValue116 = 116,
    #[doc(hidden)]
    UnknownValue117 = 117,
    #[doc(hidden)]
    UnknownValue118 = 118,
    #[doc(hidden)]
    UnknownValue119 = 119,
    #[doc(hidden)]
    UnknownValue120 = 120,
    #[doc(hidden)]
    UnknownValue121 = 121,
    #[doc(hidden)]
    UnknownValue122 = 122,
    #[doc(hidden)]
    UnknownValue123 = 123,
    #[doc(hidden)]
    UnknownValue124 = 124,
    #[doc(hidden)]
    UnknownValue125 = 125,
}

impl flatdata::helper::Int for EnumI8 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnumU8 {
    FooU8Pos = 255,
    FooU8Zero = 0,
    FooU8PosHex = 254,
    FooU8OneHex = 1,
    #[doc(hidden)]
    UnknownValue2 = 2,
    #[doc(hidden)]
    UnknownValue3 = 3,
    #[doc(hidden)]
    UnknownValue4 = 4,
    #[doc(hidden)]
    UnknownValue5 = 5,
    #[doc(hidden)]
    UnknownValue6 = 6,
    #[doc(hidden)]
    UnknownValue7 = 7,
    #[doc(hidden)]
    UnknownValue8 = 8,
    #[doc(hidden)]
    UnknownValue9 = 9,
    #[doc(hidden)]
    UnknownValue10 = 10,
    #[doc(hidden)]
    UnknownValue11 = 11,
    #[doc(hidden)]
    UnknownValue12 = 12,
    #[doc(hidden)]
    UnknownValue13 = 13,
    #[doc(hidden)]
    UnknownValue14 = 14,
    #[doc(hidden)]
    UnknownValue15 = 15,
    #[doc(hidden)]
    UnknownValue16 = 16,
    #[doc(hidden)]
    UnknownValue17 = 17,
    #[doc(hidden)]
    UnknownValue18 = 18,
    #[doc(hidden)]
    UnknownValue19 = 19,
    #[doc(hidden)]
    UnknownValue20 = 20,
    #[doc(hidden)]
    UnknownValue21 = 21,
    #[doc(hidden)]
    UnknownValue22 = 22,
    #[doc(hidden)]
    UnknownValue23 = 23,
    #[doc(hidden)]
    UnknownValue24 = 24,
    #[doc(hidden)]
    UnknownValue25 = 25,
    #[doc(hidden)]
    UnknownValue26 = 26,
    #[doc(hidden)]
    UnknownValue27 = 27,
    #[doc(hidden)]
    UnknownValue28 = 28,
    #[doc(hidden)]
    UnknownValue29 = 29,
    #[doc(hidden)]
    UnknownValue30 = 30,
    #[doc(hidden)]
    UnknownValue31 = 31,
    #[doc(hidden)]
    UnknownValue32 = 32,
    #[doc(hidden)]
    UnknownValue33 = 33,
    #[doc(hidden)]
    UnknownValue34 = 34,
    #[doc(hidden)]
    UnknownValue35 = 35,
    #[doc(hidden)]
    UnknownValue36 = 36,
    #[doc(hidden)]
    UnknownValue37 = 37,
    #[doc(hidden)]
    UnknownValue38 = 38,
    #[doc(hidden)]
    UnknownValue39 = 39,
    #[doc(hidden)]
    UnknownValue40 = 40,
    #[doc(hidden)]
    UnknownValue41 = 41,
    #[doc(hidden)]
    UnknownValue42 = 42,
    #[doc(hidden)]
    UnknownValue43 = 43,
    #[doc(hidden)]
    UnknownValue44 = 44,
    #[doc(hidden)]
    UnknownValue45 = 45,
    #[doc(hidden)]
    UnknownValue46 = 46,
    #[doc(hidden)]
    UnknownValue47 = 47,
    #[doc(hidden)]
    UnknownValue48 = 48,
    #[doc(hidden)]
    UnknownValue49 = 49,
    #[doc(hidden)]
    UnknownValue50 = 50,
    #[doc(hidden)]
    UnknownValue51 = 51,
    #[doc(hidden)]
    UnknownValue52 = 52,
    #[doc(hidden)]
    UnknownValue53 = 53,
    #[doc(hidden)]
    UnknownValue54 = 54,
    #[doc(hidden)]
    UnknownValue55 = 55,
    #[doc(hidden)]
    UnknownValue56 = 56,
    #[doc(hidden)]
    UnknownValue57 = 57,
    #[doc(hidden)]
    UnknownValue58 = 58,
    #[doc(hidden)]
    UnknownValue59 = 59,
    #[doc(hidden)]
    UnknownValue60 = 60,
    #[doc(hidden)]
    UnknownValue61 = 61,
    #[doc(hidden)]
    UnknownValue62 = 62,
    #[doc(hidden)]
    UnknownValue63 = 63,
    #[doc(hidden)]
    UnknownValue64 = 64,
    #[doc(hidden)]
    UnknownValue65 = 65,
    #[doc(hidden)]
    UnknownValue66 = 66,
    #[doc(hidden)]
    UnknownValue67 = 67,
    #[doc(hidden)]
    UnknownValue68 = 68,
    #[doc(hidden)]
    UnknownValue69 = 69,
    #[doc(hidden)]
    UnknownValue70 = 70,
    #[doc(hidden)]
    UnknownValue71 = 71,
    #[doc(hidden)]
    UnknownValue72 = 72,
    #[doc(hidden)]
    UnknownValue73 = 73,
    #[doc(hidden)]
    UnknownValue74 = 74,
    #[doc(hidden)]
    UnknownValue75 = 75,
    #[doc(hidden)]
    UnknownValue76 = 76,
    #[doc(hidden)]
    UnknownValue77 = 77,
    #[doc(hidden)]
    UnknownValue78 = 78,
    #[doc(hidden)]
    UnknownValue79 = 79,
    #[doc(hidden)]
    UnknownValue80 = 80,
    #[doc(hidden)]
    UnknownValue81 = 81,
    #[doc(hidden)]
    UnknownValue82 = 82,
    #[doc(hidden)]
    UnknownValue83 = 83,
    #[doc(hidden)]
    UnknownValue84 = 84,
    #[doc(hidden)]
    UnknownValue85 = 85,
    #[doc(hidden)]
    UnknownValue86 = 86,
    #[doc(hidden)]
    UnknownValue87 = 87,
    #[doc(hidden)]
    UnknownValue88 = 88,
    #[doc(hidden)]
    UnknownValue89 = 89,
    #[doc(hidden)]
    UnknownValue90 = 90,
    #[doc(hidden)]
    UnknownValue91 = 91,
    #[doc(hidden)]
    UnknownValue92 = 92,
    #[doc(hidden)]
    UnknownValue93 = 93,
    #[doc(hidden)]
    UnknownValue94 = 94,
    #[doc(hidden)]
    UnknownValue95 = 95,
    #[doc(hidden)]
    UnknownValue96 = 96,
    #[doc(hidden)]
    UnknownValue97 = 97,
    #[doc(hidden)]
    UnknownValue98 = 98,
    #[doc(hidden)]
    UnknownValue99 = 99,
    #[doc(hidden)]
    UnknownValue100 = 100,
    #[doc(hidden)]
    UnknownValue101 = 101,
    #[doc(hidden)]
    UnknownValue102 = 102,
    #[doc(hidden)]
    UnknownValue103 = 103,
    #[doc(hidden)]
    UnknownValue104 = 104,
    #[doc(hidden)]
    UnknownValue105 = 105,
    #[doc(hidden)]
    UnknownValue106 = 106,
    #[doc(hidden)]
    UnknownValue107 = 107,
    #[doc(hidden)]
    UnknownValue108 = 108,
    #[doc(hidden)]
    UnknownValue109 = 109,
    #[doc(hidden)]
    UnknownValue110 = 110,
    #[doc(hidden)]
    UnknownValue111 = 111,
    #[doc(hidden)]
    UnknownValue112 = 112,
    #[doc(hidden)]
    UnknownValue113 = 113,
    #[doc(hidden)]
    UnknownValue114 = 114,
    #[doc(hidden)]
    UnknownValue115 = 115,
    #[doc(hidden)]
    UnknownValue116 = 116,
    #[doc(hidden)]
    UnknownValue117 = 117,
    #[doc(hidden)]
    UnknownValue118 = 118,
    #[doc(hidden)]
    UnknownValue119 = 119,
    #[doc(hidden)]
    UnknownValue120 = 120,
    #[doc(hidden)]
    UnknownValue121 = 121,
    #[doc(hidden)]
    UnknownValue122 = 122,
    #[doc(hidden)]
    UnknownValue123 = 123,
    #[doc(hidden)]
    UnknownValue124 = 124,
    #[doc(hidden)]
    UnknownValue125 = 125,
    #[doc(hidden)]
    UnknownValue126 = 126,
    #[doc(hidden)]
    UnknownValue127 = 127,
    #[doc(hidden)]
    UnknownValue128 = 128,
    #[doc(hidden)]
    UnknownValue129 = 129,
    #[doc(hidden)]
    UnknownValue130 = 130,
    #[doc(hidden)]
    UnknownValue131 = 131,
    #[doc(hidden)]
    UnknownValue132 = 132,
    #[doc(hidden)]
    UnknownValue133 = 133,
    #[doc(hidden)]
    UnknownValue134 = 134,
    #[doc(hidden)]
    UnknownValue135 = 135,
    #[doc(hidden)]
    UnknownValue136 = 136,
    #[doc(hidden)]
    UnknownValue137 = 137,
    #[doc(hidden)]
    UnknownValue138 = 138,
    #[doc(hidden)]
    UnknownValue139 = 139,
    #[doc(hidden)]
    UnknownValue140 = 140,
    #[doc(hidden)]
    UnknownValue141 = 141,
    #[doc(hidden)]
    UnknownValue142 = 142,
    #[doc(hidden)]
    UnknownValue143 = 143,
    #[doc(hidden)]
    UnknownValue144 = 144,
    #[doc(hidden)]
    UnknownValue145 = 145,
    #[doc(hidden)]
    UnknownValue146 = 146,
    #[doc(hidden)]
    UnknownValue147 = 147,
    #[doc(hidden)]
    UnknownValue148 = 148,
    #[doc(hidden)]
    UnknownValue149 = 149,
    #[doc(hidden)]
    UnknownValue150 = 150,
    #[doc(hidden)]
    UnknownValue151 = 151,
    #[doc(hidden)]
    UnknownValue152 = 152,
    #[doc(hidden)]
    UnknownValue153 = 153,
    #[doc(hidden)]
    UnknownValue154 = 154,
    #[doc(hidden)]
    UnknownValue155 = 155,
    #[doc(hidden)]
    UnknownValue156 = 156,
    #[doc(hidden)]
    UnknownValue157 = 157,
    #[doc(hidden)]
    UnknownValue158 = 158,
    #[doc(hidden)]
    UnknownValue159 = 159,
    #[doc(hidden)]
    UnknownValue160 = 160,
    #[doc(hidden)]
    UnknownValue161 = 161,
    #[doc(hidden)]
    UnknownValue162 = 162,
    #[doc(hidden)]
    UnknownValue163 = 163,
    #[doc(hidden)]
    UnknownValue164 = 164,
    #[doc(hidden)]
    UnknownValue165 = 165,
    #[doc(hidden)]
    UnknownValue166 = 166,
    #[doc(hidden)]
    UnknownValue167 = 167,
    #[doc(hidden)]
    UnknownValue168 = 168,
    #[doc(hidden)]
    UnknownValue169 = 169,
    #[doc(hidden)]
    UnknownValue170 = 170,
    #[doc(hidden)]
    UnknownValue171 = 171,
    #[doc(hidden)]
    UnknownValue172 = 172,
    #[doc(hidden)]
    UnknownValue173 = 173,
    #[doc(hidden)]
    UnknownValue174 = 174,
    #[doc(hidden)]
    UnknownValue175 = 175,
    #[doc(hidden)]
    UnknownValue176 = 176,
    #[doc(hidden)]
    UnknownValue177 = 177,
    #[doc(hidden)]
    UnknownValue178 = 178,
    #[doc(hidden)]
    UnknownValue179 = 179,
    #[doc(hidden)]
    UnknownValue180 = 180,
    #[doc(hidden)]
    UnknownValue181 = 181,
    #[doc(hidden)]
    UnknownValue182 = 182,
    #[doc(hidden)]
    UnknownValue183 = 183,
    #[doc(hidden)]
    UnknownValue184 = 184,
    #[doc(hidden)]
    UnknownValue185 = 185,
    #[doc(hidden)]
    UnknownValue186 = 186,
    #[doc(hidden)]
    UnknownValue187 = 187,
    #[doc(hidden)]
    UnknownValue188 = 188,
    #[doc(hidden)]
    UnknownValue189 = 189,
    #[doc(hidden)]
    UnknownValue190 = 190,
    #[doc(hidden)]
    UnknownValue191 = 191,
    #[doc(hidden)]
    UnknownValue192 = 192,
    #[doc(hidden)]
    UnknownValue193 = 193,
    #[doc(hidden)]
    UnknownValue194 = 194,
    #[doc(hidden)]
    UnknownValue195 = 195,
    #[doc(hidden)]
    UnknownValue196 = 196,
    #[doc(hidden)]
    UnknownValue197 = 197,
    #[doc(hidden)]
    UnknownValue198 = 198,
    #[doc(hidden)]
    UnknownValue199 = 199,
    #[doc(hidden)]
    UnknownValue200 = 200,
    #[doc(hidden)]
    UnknownValue201 = 201,
    #[doc(hidden)]
    UnknownValue202 = 202,
    #[doc(hidden)]
    UnknownValue203 = 203,
    #[doc(hidden)]
    UnknownValue204 = 204,
    #[doc(hidden)]
    UnknownValue205 = 205,
    #[doc(hidden)]
    UnknownValue206 = 206,
    #[doc(hidden)]
    UnknownValue207 = 207,
    #[doc(hidden)]
    UnknownValue208 = 208,
    #[doc(hidden)]
    UnknownValue209 = 209,
    #[doc(hidden)]
    UnknownValue210 = 210,
    #[doc(hidden)]
    UnknownValue211 = 211,
    #[doc(hidden)]
    UnknownValue212 = 212,
    #[doc(hidden)]
    UnknownValue213 = 213,
    #[doc(hidden)]
    UnknownValue214 = 214,
    #[doc(hidden)]
    UnknownValue215 = 215,
    #[doc(hidden)]
    UnknownValue216 = 216,
    #[doc(hidden)]
    UnknownValue217 = 217,
    #[doc(hidden)]
    UnknownValue218 = 218,
    #[doc(hidden)]
    UnknownValue219 = 219,
    #[doc(hidden)]
    UnknownValue220 = 220,
    #[doc(hidden)]
    UnknownValue221 = 221,
    #[doc(hidden)]
    UnknownValue222 = 222,
    #[doc(hidden)]
    UnknownValue223 = 223,
    #[doc(hidden)]
    UnknownValue224 = 224,
    #[doc(hidden)]
    UnknownValue225 = 225,
    #[doc(hidden)]
    UnknownValue226 = 226,
    #[doc(hidden)]
    UnknownValue227 = 227,
    #[doc(hidden)]
    UnknownValue228 = 228,
    #[doc(hidden)]
    UnknownValue229 = 229,
    #[doc(hidden)]
    UnknownValue230 = 230,
    #[doc(hidden)]
    UnknownValue231 = 231,
    #[doc(hidden)]
    UnknownValue232 = 232,
    #[doc(hidden)]
    UnknownValue233 = 233,
    #[doc(hidden)]
    UnknownValue234 = 234,
    #[doc(hidden)]
    UnknownValue235 = 235,
    #[doc(hidden)]
    UnknownValue236 = 236,
    #[doc(hidden)]
    UnknownValue237 = 237,
    #[doc(hidden)]
    UnknownValue238 = 238,
    #[doc(hidden)]
    UnknownValue239 = 239,
    #[doc(hidden)]
    UnknownValue240 = 240,
    #[doc(hidden)]
    UnknownValue241 = 241,
    #[doc(hidden)]
    UnknownValue242 = 242,
    #[doc(hidden)]
    UnknownValue243 = 243,
    #[doc(hidden)]
    UnknownValue244 = 244,
    #[doc(hidden)]
    UnknownValue245 = 245,
    #[doc(hidden)]
    UnknownValue246 = 246,
    #[doc(hidden)]
    UnknownValue247 = 247,
    #[doc(hidden)]
    UnknownValue248 = 248,
    #[doc(hidden)]
    UnknownValue249 = 249,
    #[doc(hidden)]
    UnknownValue250 = 250,
    #[doc(hidden)]
    UnknownValue251 = 251,
    #[doc(hidden)]
    UnknownValue252 = 252,
    #[doc(hidden)]
    UnknownValue253 = 253,
}

impl flatdata::helper::Int for EnumU8 {
    const IS_SIGNED: bool = false;
}
}