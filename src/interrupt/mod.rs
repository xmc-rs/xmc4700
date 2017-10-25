use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm!("\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    ");
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm!("\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    ");
#[cfg(feature = "rt")]
global_asm!(
    "\n.weak SCU_0\nSCU_0 = DH_TRAMPOLINE\n.weak ERU0_0\nERU0_0 = DH_TRAMPOLINE\n.weak ERU0_1\nERU0_1 = DH_TRAMPOLINE\n.weak ERU0_2\nERU0_2 = DH_TRAMPOLINE\n.weak ERU0_3\nERU0_3 = DH_TRAMPOLINE\n.weak ERU1_0\nERU1_0 = DH_TRAMPOLINE\n.weak ERU1_1\nERU1_1 = DH_TRAMPOLINE\n.weak ERU1_2\nERU1_2 = DH_TRAMPOLINE\n.weak ERU1_3\nERU1_3 = DH_TRAMPOLINE\n.weak PMU0_0\nPMU0_0 = DH_TRAMPOLINE\n.weak VADC0_C0_0\nVADC0_C0_0 = DH_TRAMPOLINE\n.weak VADC0_C0_1\nVADC0_C0_1 = DH_TRAMPOLINE\n.weak VADC0_C0_2\nVADC0_C0_2 = DH_TRAMPOLINE\n.weak VADC0_C0_3\nVADC0_C0_3 = DH_TRAMPOLINE\n.weak VADC0_G0_0\nVADC0_G0_0 = DH_TRAMPOLINE\n.weak VADC0_G0_1\nVADC0_G0_1 = DH_TRAMPOLINE\n.weak VADC0_G0_2\nVADC0_G0_2 = DH_TRAMPOLINE\n.weak VADC0_G0_3\nVADC0_G0_3 = DH_TRAMPOLINE\n.weak VADC0_G1_0\nVADC0_G1_0 = DH_TRAMPOLINE\n.weak VADC0_G1_1\nVADC0_G1_1 = DH_TRAMPOLINE\n.weak VADC0_G1_2\nVADC0_G1_2 = DH_TRAMPOLINE\n.weak VADC0_G1_3\nVADC0_G1_3 = DH_TRAMPOLINE\n.weak VADC0_G2_0\nVADC0_G2_0 = DH_TRAMPOLINE\n.weak VADC0_G2_1\nVADC0_G2_1 = DH_TRAMPOLINE\n.weak VADC0_G2_2\nVADC0_G2_2 = DH_TRAMPOLINE\n.weak VADC0_G2_3\nVADC0_G2_3 = DH_TRAMPOLINE\n.weak VADC0_G3_0\nVADC0_G3_0 = DH_TRAMPOLINE\n.weak VADC0_G3_1\nVADC0_G3_1 = DH_TRAMPOLINE\n.weak VADC0_G3_2\nVADC0_G3_2 = DH_TRAMPOLINE\n.weak VADC0_G3_3\nVADC0_G3_3 = DH_TRAMPOLINE\n.weak DSD0_M_0\nDSD0_M_0 = DH_TRAMPOLINE\n.weak DSD0_M_1\nDSD0_M_1 = DH_TRAMPOLINE\n.weak DSD0_M_2\nDSD0_M_2 = DH_TRAMPOLINE\n.weak DSD0_M_3\nDSD0_M_3 = DH_TRAMPOLINE\n.weak DSD0_A_4\nDSD0_A_4 = DH_TRAMPOLINE\n.weak DSD0_A_5\nDSD0_A_5 = DH_TRAMPOLINE\n.weak DSD0_A_6\nDSD0_A_6 = DH_TRAMPOLINE\n.weak DSD0_A_7\nDSD0_A_7 = DH_TRAMPOLINE\n.weak DAC0_0\nDAC0_0 = DH_TRAMPOLINE\n.weak DAC0_1\nDAC0_1 = DH_TRAMPOLINE\n.weak CCU40_0\nCCU40_0 = DH_TRAMPOLINE\n.weak CCU40_1\nCCU40_1 = DH_TRAMPOLINE\n.weak CCU40_2\nCCU40_2 = DH_TRAMPOLINE\n.weak CCU40_3\nCCU40_3 = DH_TRAMPOLINE\n.weak CCU41_0\nCCU41_0 = DH_TRAMPOLINE\n.weak CCU41_1\nCCU41_1 = DH_TRAMPOLINE\n.weak CCU41_2\nCCU41_2 = DH_TRAMPOLINE\n.weak CCU41_3\nCCU41_3 = DH_TRAMPOLINE\n.weak CCU42_0\nCCU42_0 = DH_TRAMPOLINE\n.weak CCU42_1\nCCU42_1 = DH_TRAMPOLINE\n.weak CCU42_2\nCCU42_2 = DH_TRAMPOLINE\n.weak CCU42_3\nCCU42_3 = DH_TRAMPOLINE\n.weak CCU43_0\nCCU43_0 = DH_TRAMPOLINE\n.weak CCU43_1\nCCU43_1 = DH_TRAMPOLINE\n.weak CCU43_2\nCCU43_2 = DH_TRAMPOLINE\n.weak CCU43_3\nCCU43_3 = DH_TRAMPOLINE\n.weak CCU80_0\nCCU80_0 = DH_TRAMPOLINE\n.weak CCU80_1\nCCU80_1 = DH_TRAMPOLINE\n.weak CCU80_2\nCCU80_2 = DH_TRAMPOLINE\n.weak CCU80_3\nCCU80_3 = DH_TRAMPOLINE\n.weak CCU81_0\nCCU81_0 = DH_TRAMPOLINE\n.weak CCU81_1\nCCU81_1 = DH_TRAMPOLINE\n.weak CCU81_2\nCCU81_2 = DH_TRAMPOLINE\n.weak CCU81_3\nCCU81_3 = DH_TRAMPOLINE\n.weak POSIF0_0\nPOSIF0_0 = DH_TRAMPOLINE\n.weak POSIF0_1\nPOSIF0_1 = DH_TRAMPOLINE\n.weak POSIF1_0\nPOSIF1_0 = DH_TRAMPOLINE\n.weak POSIF1_1\nPOSIF1_1 = DH_TRAMPOLINE\n.weak CAN0_0\nCAN0_0 = DH_TRAMPOLINE\n.weak CAN0_1\nCAN0_1 = DH_TRAMPOLINE\n.weak CAN0_2\nCAN0_2 = DH_TRAMPOLINE\n.weak CAN0_3\nCAN0_3 = DH_TRAMPOLINE\n.weak CAN0_4\nCAN0_4 = DH_TRAMPOLINE\n.weak CAN0_5\nCAN0_5 = DH_TRAMPOLINE\n.weak CAN0_6\nCAN0_6 = DH_TRAMPOLINE\n.weak CAN0_7\nCAN0_7 = DH_TRAMPOLINE\n.weak USIC0_0\nUSIC0_0 = DH_TRAMPOLINE\n.weak USIC0_1\nUSIC0_1 = DH_TRAMPOLINE\n.weak USIC0_2\nUSIC0_2 = DH_TRAMPOLINE\n.weak USIC0_3\nUSIC0_3 = DH_TRAMPOLINE\n.weak USIC0_4\nUSIC0_4 = DH_TRAMPOLINE\n.weak USIC0_5\nUSIC0_5 = DH_TRAMPOLINE\n.weak USIC1_0\nUSIC1_0 = DH_TRAMPOLINE\n.weak USIC1_1\nUSIC1_1 = DH_TRAMPOLINE\n.weak USIC1_2\nUSIC1_2 = DH_TRAMPOLINE\n.weak USIC1_3\nUSIC1_3 = DH_TRAMPOLINE\n.weak USIC1_4\nUSIC1_4 = DH_TRAMPOLINE\n.weak USIC1_5\nUSIC1_5 = DH_TRAMPOLINE\n.weak USIC2_0\nUSIC2_0 = DH_TRAMPOLINE\n.weak USIC2_1\nUSIC2_1 = DH_TRAMPOLINE\n.weak USIC2_2\nUSIC2_2 = DH_TRAMPOLINE\n.weak USIC2_3\nUSIC2_3 = DH_TRAMPOLINE\n.weak USIC2_4\nUSIC2_4 = DH_TRAMPOLINE\n.weak USIC2_5\nUSIC2_5 = DH_TRAMPOLINE\n.weak LEDTS0_0\nLEDTS0_0 = DH_TRAMPOLINE\n.weak FCE0_0\nFCE0_0 = DH_TRAMPOLINE\n.weak GPDMA0_0\nGPDMA0_0 = DH_TRAMPOLINE\n.weak SDMMC0_0\nSDMMC0_0 = DH_TRAMPOLINE\n.weak USB0_0\nUSB0_0 = DH_TRAMPOLINE\n.weak ETH0_0\nETH0_0 = DH_TRAMPOLINE\n.weak GPDMA1_0\nGPDMA1_0 = DH_TRAMPOLINE"
);
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn ERU1_0();
    fn ERU1_1();
    fn ERU1_2();
    fn ERU1_3();
    fn PMU0_0();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn VADC0_C0_2();
    fn VADC0_C0_3();
    fn VADC0_G0_0();
    fn VADC0_G0_1();
    fn VADC0_G0_2();
    fn VADC0_G0_3();
    fn VADC0_G1_0();
    fn VADC0_G1_1();
    fn VADC0_G1_2();
    fn VADC0_G1_3();
    fn VADC0_G2_0();
    fn VADC0_G2_1();
    fn VADC0_G2_2();
    fn VADC0_G2_3();
    fn VADC0_G3_0();
    fn VADC0_G3_1();
    fn VADC0_G3_2();
    fn VADC0_G3_3();
    fn DSD0_M_0();
    fn DSD0_M_1();
    fn DSD0_M_2();
    fn DSD0_M_3();
    fn DSD0_A_4();
    fn DSD0_A_5();
    fn DSD0_A_6();
    fn DSD0_A_7();
    fn DAC0_0();
    fn DAC0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
    fn CCU41_0();
    fn CCU41_1();
    fn CCU41_2();
    fn CCU41_3();
    fn CCU42_0();
    fn CCU42_1();
    fn CCU42_2();
    fn CCU42_3();
    fn CCU43_0();
    fn CCU43_1();
    fn CCU43_2();
    fn CCU43_3();
    fn CCU80_0();
    fn CCU80_1();
    fn CCU80_2();
    fn CCU80_3();
    fn CCU81_0();
    fn CCU81_1();
    fn CCU81_2();
    fn CCU81_3();
    fn POSIF0_0();
    fn POSIF0_1();
    fn POSIF1_0();
    fn POSIF1_1();
    fn CAN0_0();
    fn CAN0_1();
    fn CAN0_2();
    fn CAN0_3();
    fn CAN0_4();
    fn CAN0_5();
    fn CAN0_6();
    fn CAN0_7();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn USIC1_0();
    fn USIC1_1();
    fn USIC1_2();
    fn USIC1_3();
    fn USIC1_4();
    fn USIC1_5();
    fn USIC2_0();
    fn USIC2_1();
    fn USIC2_2();
    fn USIC2_3();
    fn USIC2_4();
    fn USIC2_5();
    fn LEDTS0_0();
    fn FCE0_0();
    fn GPDMA0_0();
    fn SDMMC0_0();
    fn USB0_0();
    fn ETH0_0();
    fn GPDMA1_0();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 111] = [
    Some(SCU_0),
    Some(ERU0_0),
    Some(ERU0_1),
    Some(ERU0_2),
    Some(ERU0_3),
    Some(ERU1_0),
    Some(ERU1_1),
    Some(ERU1_2),
    Some(ERU1_3),
    None,
    None,
    None,
    Some(PMU0_0),
    None,
    Some(VADC0_C0_0),
    Some(VADC0_C0_1),
    Some(VADC0_C0_2),
    Some(VADC0_C0_3),
    Some(VADC0_G0_0),
    Some(VADC0_G0_1),
    Some(VADC0_G0_2),
    Some(VADC0_G0_3),
    Some(VADC0_G1_0),
    Some(VADC0_G1_1),
    Some(VADC0_G1_2),
    Some(VADC0_G1_3),
    Some(VADC0_G2_0),
    Some(VADC0_G2_1),
    Some(VADC0_G2_2),
    Some(VADC0_G2_3),
    Some(VADC0_G3_0),
    Some(VADC0_G3_1),
    Some(VADC0_G3_2),
    Some(VADC0_G3_3),
    Some(DSD0_M_0),
    Some(DSD0_M_1),
    Some(DSD0_M_2),
    Some(DSD0_M_3),
    Some(DSD0_A_4),
    Some(DSD0_A_5),
    Some(DSD0_A_6),
    Some(DSD0_A_7),
    Some(DAC0_0),
    Some(DAC0_1),
    Some(CCU40_0),
    Some(CCU40_1),
    Some(CCU40_2),
    Some(CCU40_3),
    Some(CCU41_0),
    Some(CCU41_1),
    Some(CCU41_2),
    Some(CCU41_3),
    Some(CCU42_0),
    Some(CCU42_1),
    Some(CCU42_2),
    Some(CCU42_3),
    Some(CCU43_0),
    Some(CCU43_1),
    Some(CCU43_2),
    Some(CCU43_3),
    Some(CCU80_0),
    Some(CCU80_1),
    Some(CCU80_2),
    Some(CCU80_3),
    Some(CCU81_0),
    Some(CCU81_1),
    Some(CCU81_2),
    Some(CCU81_3),
    Some(POSIF0_0),
    Some(POSIF0_1),
    Some(POSIF1_0),
    Some(POSIF1_1),
    None,
    None,
    None,
    None,
    Some(CAN0_0),
    Some(CAN0_1),
    Some(CAN0_2),
    Some(CAN0_3),
    Some(CAN0_4),
    Some(CAN0_5),
    Some(CAN0_6),
    Some(CAN0_7),
    Some(USIC0_0),
    Some(USIC0_1),
    Some(USIC0_2),
    Some(USIC0_3),
    Some(USIC0_4),
    Some(USIC0_5),
    Some(USIC1_0),
    Some(USIC1_1),
    Some(USIC1_2),
    Some(USIC1_3),
    Some(USIC1_4),
    Some(USIC1_5),
    Some(USIC2_0),
    Some(USIC2_1),
    Some(USIC2_2),
    Some(USIC2_3),
    Some(USIC2_4),
    Some(USIC2_5),
    Some(LEDTS0_0),
    None,
    Some(FCE0_0),
    Some(GPDMA0_0),
    Some(SDMMC0_0),
    Some(USB0_0),
    Some(ETH0_0),
    None,
    Some(GPDMA1_0),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0,
    #[doc = "1 - External Request Unit 0"]
    ERU0_0,
    #[doc = "2 - External Request Unit 0"]
    ERU0_1,
    #[doc = "3 - External Request Unit 0"]
    ERU0_2,
    #[doc = "4 - External Request Unit 0"]
    ERU0_3,
    #[doc = "5 - External Request Unit 1"]
    ERU1_0,
    #[doc = "6 - External Request Unit 1"]
    ERU1_1,
    #[doc = "7 - External Request Unit 1"]
    ERU1_2,
    #[doc = "8 - External Request Unit 1"]
    ERU1_3,
    #[doc = "12 - Program Management Unit"]
    PMU0_0,
    #[doc = "14 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_2,
    #[doc = "17 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_3,
    #[doc = "18 - Analog to Digital Converter Group 0"]
    VADC0_G0_0,
    #[doc = "19 - Analog to Digital Converter Group 0"]
    VADC0_G0_1,
    #[doc = "20 - Analog to Digital Converter Group 0"]
    VADC0_G0_2,
    #[doc = "21 - Analog to Digital Converter Group 0"]
    VADC0_G0_3,
    #[doc = "22 - Analog to Digital Converter Group 1"]
    VADC0_G1_0,
    #[doc = "23 - Analog to Digital Converter Group 1"]
    VADC0_G1_1,
    #[doc = "24 - Analog to Digital Converter Group 1"]
    VADC0_G1_2,
    #[doc = "25 - Analog to Digital Converter Group 1"]
    VADC0_G1_3,
    #[doc = "26 - Analog to Digital Converter Group 2"]
    VADC0_G2_0,
    #[doc = "27 - Analog to Digital Converter Group 2"]
    VADC0_G2_1,
    #[doc = "28 - Analog to Digital Converter Group 2"]
    VADC0_G2_2,
    #[doc = "29 - Analog to Digital Converter Group 2"]
    VADC0_G2_3,
    #[doc = "30 - Analog to Digital Converter Group 3"]
    VADC0_G3_0,
    #[doc = "31 - Analog to Digital Converter Group 3"]
    VADC0_G3_1,
    #[doc = "32 - Analog to Digital Converter Group 3"]
    VADC0_G3_2,
    #[doc = "33 - Analog to Digital Converter Group 3"]
    VADC0_G3_3,
    #[doc = "34 - Delta Sigma Demodulator Main"]
    DSD0_M_0,
    #[doc = "35 - Delta Sigma Demodulator Main"]
    DSD0_M_1,
    #[doc = "36 - Delta Sigma Demodulator Main"]
    DSD0_M_2,
    #[doc = "37 - Delta Sigma Demodulator Main"]
    DSD0_M_3,
    #[doc = "38 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_4,
    #[doc = "39 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_5,
    #[doc = "40 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_6,
    #[doc = "41 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_7,
    #[doc = "42 - Digital to Analog Converter"]
    DAC0_0,
    #[doc = "43 - Digital to Analog Converter"]
    DAC0_1,
    #[doc = "44 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0,
    #[doc = "45 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1,
    #[doc = "46 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2,
    #[doc = "47 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3,
    #[doc = "48 - Capture Compare Unit 4 (Module 1)"]
    CCU41_0,
    #[doc = "49 - Capture Compare Unit 4 (Module 1)"]
    CCU41_1,
    #[doc = "50 - Capture Compare Unit 4 (Module 1)"]
    CCU41_2,
    #[doc = "51 - Capture Compare Unit 4 (Module 1)"]
    CCU41_3,
    #[doc = "52 - Capture Compare Unit 4 (Module 2)"]
    CCU42_0,
    #[doc = "53 - Capture Compare Unit 4 (Module 2)"]
    CCU42_1,
    #[doc = "54 - Capture Compare Unit 4 (Module 2)"]
    CCU42_2,
    #[doc = "55 - Capture Compare Unit 4 (Module 2)"]
    CCU42_3,
    #[doc = "56 - Capture Compare Unit 4 (Module 3)"]
    CCU43_0,
    #[doc = "57 - Capture Compare Unit 4 (Module 3)"]
    CCU43_1,
    #[doc = "58 - Capture Compare Unit 4 (Module 3)"]
    CCU43_2,
    #[doc = "59 - Capture Compare Unit 4 (Module 3)"]
    CCU43_3,
    #[doc = "60 - Capture Compare Unit 8 (Module 0)"]
    CCU80_0,
    #[doc = "61 - Capture Compare Unit 8 (Module 0)"]
    CCU80_1,
    #[doc = "62 - Capture Compare Unit 8 (Module 0)"]
    CCU80_2,
    #[doc = "63 - Capture Compare Unit 8 (Module 0)"]
    CCU80_3,
    #[doc = "64 - Capture Compare Unit 8 (Module 1)"]
    CCU81_0,
    #[doc = "65 - Capture Compare Unit 8 (Module 1)"]
    CCU81_1,
    #[doc = "66 - Capture Compare Unit 8 (Module 1)"]
    CCU81_2,
    #[doc = "67 - Capture Compare Unit 8 (Module 1)"]
    CCU81_3,
    #[doc = "68 - Position Interface (Module 0)"]
    POSIF0_0,
    #[doc = "69 - Position Interface (Module 0)"]
    POSIF0_1,
    #[doc = "70 - Position Interface (Module 1)"]
    POSIF1_0,
    #[doc = "71 - Position Interface (Module 1)"]
    POSIF1_1,
    #[doc = "76 - MultiCAN"]
    CAN0_0,
    #[doc = "77 - MultiCAN"]
    CAN0_1,
    #[doc = "78 - MultiCAN"]
    CAN0_2,
    #[doc = "79 - MultiCAN"]
    CAN0_3,
    #[doc = "80 - MultiCAN"]
    CAN0_4,
    #[doc = "81 - MultiCAN"]
    CAN0_5,
    #[doc = "82 - MultiCAN"]
    CAN0_6,
    #[doc = "83 - MultiCAN"]
    CAN0_7,
    #[doc = "84 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0,
    #[doc = "85 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1,
    #[doc = "86 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2,
    #[doc = "87 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3,
    #[doc = "88 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4,
    #[doc = "89 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5,
    #[doc = "90 - Universal Serial Interface Channel (Module 1)"]
    USIC1_0,
    #[doc = "91 - Universal Serial Interface Channel (Module 1)"]
    USIC1_1,
    #[doc = "92 - Universal Serial Interface Channel (Module 1)"]
    USIC1_2,
    #[doc = "93 - Universal Serial Interface Channel (Module 1)"]
    USIC1_3,
    #[doc = "94 - Universal Serial Interface Channel (Module 1)"]
    USIC1_4,
    #[doc = "95 - Universal Serial Interface Channel (Module 1)"]
    USIC1_5,
    #[doc = "96 - Universal Serial Interface Channel (Module 2)"]
    USIC2_0,
    #[doc = "97 - Universal Serial Interface Channel (Module 2)"]
    USIC2_1,
    #[doc = "98 - Universal Serial Interface Channel (Module 2)"]
    USIC2_2,
    #[doc = "99 - Universal Serial Interface Channel (Module 2)"]
    USIC2_3,
    #[doc = "100 - Universal Serial Interface Channel (Module 2)"]
    USIC2_4,
    #[doc = "101 - Universal Serial Interface Channel (Module 2)"]
    USIC2_5,
    #[doc = "102 - LED and Touch Sense Control Unit (Module 0)"]
    LEDTS0_0,
    #[doc = "104 - Flexible CRC Engine"]
    FCE0_0,
    #[doc = "105 - General Purpose DMA Unit 0"]
    GPDMA0_0,
    #[doc = "106 - Multi Media Card Interface"]
    SDMMC0_0,
    #[doc = "107 - Universal Serial Bus (Module 0)"]
    USB0_0,
    #[doc = "108 - Ethernet (Module 0)"]
    ETH0_0,
    #[doc = "110 - General Purpose DMA Unit 1"]
    GPDMA1_0,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SCU_0 => 0,
            Interrupt::ERU0_0 => 1,
            Interrupt::ERU0_1 => 2,
            Interrupt::ERU0_2 => 3,
            Interrupt::ERU0_3 => 4,
            Interrupt::ERU1_0 => 5,
            Interrupt::ERU1_1 => 6,
            Interrupt::ERU1_2 => 7,
            Interrupt::ERU1_3 => 8,
            Interrupt::PMU0_0 => 12,
            Interrupt::VADC0_C0_0 => 14,
            Interrupt::VADC0_C0_1 => 15,
            Interrupt::VADC0_C0_2 => 16,
            Interrupt::VADC0_C0_3 => 17,
            Interrupt::VADC0_G0_0 => 18,
            Interrupt::VADC0_G0_1 => 19,
            Interrupt::VADC0_G0_2 => 20,
            Interrupt::VADC0_G0_3 => 21,
            Interrupt::VADC0_G1_0 => 22,
            Interrupt::VADC0_G1_1 => 23,
            Interrupt::VADC0_G1_2 => 24,
            Interrupt::VADC0_G1_3 => 25,
            Interrupt::VADC0_G2_0 => 26,
            Interrupt::VADC0_G2_1 => 27,
            Interrupt::VADC0_G2_2 => 28,
            Interrupt::VADC0_G2_3 => 29,
            Interrupt::VADC0_G3_0 => 30,
            Interrupt::VADC0_G3_1 => 31,
            Interrupt::VADC0_G3_2 => 32,
            Interrupt::VADC0_G3_3 => 33,
            Interrupt::DSD0_M_0 => 34,
            Interrupt::DSD0_M_1 => 35,
            Interrupt::DSD0_M_2 => 36,
            Interrupt::DSD0_M_3 => 37,
            Interrupt::DSD0_A_4 => 38,
            Interrupt::DSD0_A_5 => 39,
            Interrupt::DSD0_A_6 => 40,
            Interrupt::DSD0_A_7 => 41,
            Interrupt::DAC0_0 => 42,
            Interrupt::DAC0_1 => 43,
            Interrupt::CCU40_0 => 44,
            Interrupt::CCU40_1 => 45,
            Interrupt::CCU40_2 => 46,
            Interrupt::CCU40_3 => 47,
            Interrupt::CCU41_0 => 48,
            Interrupt::CCU41_1 => 49,
            Interrupt::CCU41_2 => 50,
            Interrupt::CCU41_3 => 51,
            Interrupt::CCU42_0 => 52,
            Interrupt::CCU42_1 => 53,
            Interrupt::CCU42_2 => 54,
            Interrupt::CCU42_3 => 55,
            Interrupt::CCU43_0 => 56,
            Interrupt::CCU43_1 => 57,
            Interrupt::CCU43_2 => 58,
            Interrupt::CCU43_3 => 59,
            Interrupt::CCU80_0 => 60,
            Interrupt::CCU80_1 => 61,
            Interrupt::CCU80_2 => 62,
            Interrupt::CCU80_3 => 63,
            Interrupt::CCU81_0 => 64,
            Interrupt::CCU81_1 => 65,
            Interrupt::CCU81_2 => 66,
            Interrupt::CCU81_3 => 67,
            Interrupt::POSIF0_0 => 68,
            Interrupt::POSIF0_1 => 69,
            Interrupt::POSIF1_0 => 70,
            Interrupt::POSIF1_1 => 71,
            Interrupt::CAN0_0 => 76,
            Interrupt::CAN0_1 => 77,
            Interrupt::CAN0_2 => 78,
            Interrupt::CAN0_3 => 79,
            Interrupt::CAN0_4 => 80,
            Interrupt::CAN0_5 => 81,
            Interrupt::CAN0_6 => 82,
            Interrupt::CAN0_7 => 83,
            Interrupt::USIC0_0 => 84,
            Interrupt::USIC0_1 => 85,
            Interrupt::USIC0_2 => 86,
            Interrupt::USIC0_3 => 87,
            Interrupt::USIC0_4 => 88,
            Interrupt::USIC0_5 => 89,
            Interrupt::USIC1_0 => 90,
            Interrupt::USIC1_1 => 91,
            Interrupt::USIC1_2 => 92,
            Interrupt::USIC1_3 => 93,
            Interrupt::USIC1_4 => 94,
            Interrupt::USIC1_5 => 95,
            Interrupt::USIC2_0 => 96,
            Interrupt::USIC2_1 => 97,
            Interrupt::USIC2_2 => 98,
            Interrupt::USIC2_3 => 99,
            Interrupt::USIC2_4 => 100,
            Interrupt::USIC2_5 => 101,
            Interrupt::LEDTS0_0 => 102,
            Interrupt::FCE0_0 => 104,
            Interrupt::GPDMA0_0 => 105,
            Interrupt::SDMMC0_0 => 106,
            Interrupt::USB0_0 => 107,
            Interrupt::ETH0_0 => 108,
            Interrupt::GPDMA1_0 => 110,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ident = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
