//! PL011 UART register definitions.
//!
//! Documentation: ARM PrimeCell UART (PL011) Technical Reference Manual (ARM DDI 0183G), Chapter 3

use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

register_bitfields![u32,
    /// Data Register, UARTDR.
    pub DR [
        /// Overrun error. This bit is set to 1 if data is received and the
        /// receive FIFO is already full.
        OE OFFSET(11) NUMBITS(1) [],
        /// Break error. This bit is set to 1 if a break condition was
        /// detected, indicating that the received data input was held LOW for
        /// longer than a full-word transmission time (defined as start, data,
        /// parity and stop bits).
        BE OFFSET(10) NUMBITS(1) [],
        /// Parity error. When set to 1, it indicates that the parity of the
        /// received data character does not match the parity that the EPS and
        /// SPS bits in the Line Control Register, UARTLCR_H select.
        PE OFFSET(9) NUMBITS(1) [],
        /// Framing error. When set to 1, it indicates that the received
        /// character did not have a valid stop bit (a valid stop bit is 1).
        FE OFFSET(8) NUMBITS(1) [],
        /// Receive (read) data character. Transmit (write) data character.
        DATA OFFSET(0) NUMBITS(8) []
    ],

    /// Receive Status Register / Error Clear Register, UARTRSR/UARTECR.
    pub RSR_ECR [
        /// Overrun error.
        OE OFFSET(3) NUMBITS(1) [],
        /// Break error.
        BE OFFSET(2) NUMBITS(1) [],
        /// Parity error.
        PE OFFSET(1) NUMBITS(1) [],
        /// Framing error.
        FE OFFSET(0) NUMBITS(1) []
    ],

    /// Flag Register, UARTFR.
    pub FR [
        /// Ring indicator.
        RI OFFSET(8) NUMBITS(1) [],
        /// Transmit FIFO empty.
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Receive FIFO full.
        RXFF OFFSET(6) NUMBITS(1) [],
        /// Transmit FIFO full.
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO empty.
        RXFE OFFSET(4) NUMBITS(1) [],
        /// UART busy. If this bit is set to 1, the UART is busy transmitting
        /// data.
        BUSY OFFSET(3) NUMBITS(1) [],
        /// Data carrier detect.
        DCD OFFSET(2) NUMBITS(1) [],
        /// Data set ready.
        DSR OFFSET(1) NUMBITS(1) [],
        /// Clear to send.
        CTS OFFSET(0) NUMBITS(1) []
    ],

    /// IrDA Low-Power Counter Register, UARTILPR.
    pub ILPR [
        /// 8-bit low-power divisor value.
        ILPDVSR OFFSET(0) NUMBITS(8) []
    ],

    /// Integer Baud Rate Register, UARTIBRD.
    pub IBRD [
        /// The integer baud rate divisor.
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Register, UARTFBRD.
    pub FBRD [
        /// The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register, UARTLCR_H.
    pub LCR_H [
        /// Stick parity select.
        SPS OFFSET(7) NUMBITS(1) [],
        /// Word length.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],
        /// Enable FIFOs.
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Two stop bits select.
        STP2 OFFSET(3) NUMBITS(1) [],
        /// Even parity select.
        EPS OFFSET(2) NUMBITS(1) [
            Odd = 0,
            Even = 1
        ],
        /// Parity enable.
        PEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Send break.
        BRK OFFSET(0) NUMBITS(1) []
    ],

    /// Control Register, UARTCR.
    pub CR [
        /// CTS hardware flow control enable.
        CTSEN OFFSET(15) NUMBITS(1) [],
        /// RTS hardware flow control enable.
        RTSEN OFFSET(14) NUMBITS(1) [],
        /// This bit is the complement of the UART Out2 (nUARTOut2) modem
        /// status output.
        OUT2 OFFSET(13) NUMBITS(1) [],
        /// This bit is the complement of the UART Out1 (nUARTOut1) modem
        /// status output.
        OUT1 OFFSET(12) NUMBITS(1) [],
        /// Request to send.
        RTS OFFSET(11) NUMBITS(1) [],
        /// Data transmit ready.
        DTR OFFSET(10) NUMBITS(1) [],
        /// Receive enable.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Transmit enable.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Loopback enable.
        LBE OFFSET(7) NUMBITS(1) [],
        /// SIR low-power IrDA mode.
        SIRLP OFFSET(2) NUMBITS(1) [],
        /// SIR enable.
        SIREN OFFSET(1) NUMBITS(1) [],
        /// UART enable.
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt FIFO Level Select Register, UARTIFLS.
    pub IFLS [
        /// Receive interrupt FIFO level select.
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            OneEighth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEighths = 0b100
        ],
        /// Transmit interrupt FIFO level select.
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            OneEighth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEighths = 0b100
        ]
    ],

    /// Interrupt Mask Set/Clear Register, UARTIMSC.
    /// UARTRIS, UARTMIS and UARTICR share the same layout
    pub INTERRUPTS [
        /// Overrun error interrupt.
        OEI OFFSET(10) NUMBITS(1) [],
        /// Break error interrupt.
        BEI OFFSET(9) NUMBITS(1) [],
        /// Parity error interrupt.
        PEI OFFSET(8) NUMBITS(1) [],
        /// Framing error interrupt.
        FEI OFFSET(7) NUMBITS(1) [],
        /// Receive timeout interrupt.
        RTI OFFSET(6) NUMBITS(1) [],
        /// Transmit interrupt.
        TXI OFFSET(5) NUMBITS(1) [],
        /// Receive interrupt.
        RXI OFFSET(4) NUMBITS(1) [],
        /// nUARTDSR modem interrupt.
        DSRMI OFFSET(3) NUMBITS(1) [],
        /// nUARTDCD modem interrupt.
        DCDMI OFFSET(2) NUMBITS(1) [],
        /// nUARTCTS modem interrupt.
        CTSMI OFFSET(1) NUMBITS(1) [],
        /// nUARTRI modem interrupt.
        RIMI OFFSET(0) NUMBITS(1) []
    ],

    /// DMA Control Register, UARTDMACR.
    pub DMACR [
        /// DMA on error. If this bit is set to 1, the DMA receive request
        /// outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the
        /// UART error interrupt is asserted.
        DMAONERR OFFSET(2) NUMBITS(1) [],
        /// Transmit DMA enable.
        TXDMAE OFFSET(1) NUMBITS(1) [],
        /// Receive DMA enable.
        RXDMAE OFFSET(0) NUMBITS(1) []
    ]
];

register_structs! {
    /// PL011 UART register block.
    pub Pl011Registers {
        /// Data Register.
        (0x000 => pub dr: ReadWrite<u32, DR::Register>),
        /// Receive Status Register / Error Clear Register.
        (0x004 => pub rsr_ecr: ReadWrite<u32, RSR_ECR::Register>),
        (0x008 => _reserved0),
        /// Flag Register.
        (0x018 => pub fr: ReadOnly<u32, FR::Register>),
        (0x01c => _reserved1),
        /// IrDA Low-Power Counter Register.
        (0x020 => pub ilpr: ReadWrite<u32, ILPR::Register>),
        /// Integer Baud Rate Register.
        (0x024 => pub ibrd: ReadWrite<u32, IBRD::Register>),
        /// Fractional Baud Rate Register.
        (0x028 => pub fbrd: ReadWrite<u32, FBRD::Register>),
        /// Line Control Register.
        (0x02c => pub lcr_h: ReadWrite<u32, LCR_H::Register>),
        /// Control Register.
        (0x030 => pub cr: ReadWrite<u32, CR::Register>),
        /// Interrupt FIFO Level Select Register.
        (0x034 => pub ifls: ReadWrite<u32, IFLS::Register>),
        /// Interrupt Mask Set/Clear Register.
        (0x038 => pub imsc: ReadWrite<u32, INTERRUPTS::Register>),
        /// Raw Interrupt Status Register.
        (0x03c => pub ris: ReadOnly<u32, INTERRUPTS::Register>),
        /// Masked Interrupt Status Register.
        (0x040 => pub mis: ReadOnly<u32, INTERRUPTS::Register>),
        /// Interrupt Clear Register.
        (0x044 => pub icr: WriteOnly<u32, INTERRUPTS::Register>),
        /// DMA Control Register.
        (0x048 => pub dmacr: ReadWrite<u32, DMACR::Register>),
        (0x04c => _reserved2),
        /// Peripheral Identification Registers.
        (0xfe0 => pub periph_id: [ReadOnly<u32>; 4]),
        /// PrimeCell Identification Registers.
        (0xff0 => pub pcell_id: [ReadOnly<u32>; 4]),
        (0x1000 => @END),
    }
}
