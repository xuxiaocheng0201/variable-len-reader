macro_rules! define_bools_read {
    () => {
        #[inline]
        fn read_bools_2(&mut self) -> Result<(bool, bool)> {
            let b = self.read()?;
            if b > 0b11 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b01 != 0;
            let b2 = b & 0b10 != 0;
            Ok((b1, b2))
        }
        #[inline]
        fn read_bools_3(&mut self) -> Result<(bool, bool, bool)> {
            let b = self.read()?;
            if b > 0b111 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b001 != 0;
            let b2 = b & 0b010 != 0;
            let b3 = b & 0b100 != 0;
            Ok((b1, b2, b3))
        }
        #[inline]
        fn read_bools_4(&mut self) -> Result<(bool, bool, bool, bool)> {
            let b = self.read()?;
            if b > 0b1111 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b0001 != 0;
            let b2 = b & 0b0010 != 0;
            let b3 = b & 0b0100 != 0;
            let b4 = b & 0b1000 != 0;
            Ok((b1, b2, b3, b4))
        }
        #[inline]
        fn read_bools_5(&mut self) -> Result<(bool, bool, bool, bool, bool)> {
            let b = self.read()?;
            if b > 0b11111 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b00001 != 0;
            let b2 = b & 0b00010 != 0;
            let b3 = b & 0b00100 != 0;
            let b4 = b & 0b01000 != 0;
            let b5 = b & 0b10000 != 0;
            Ok((b1, b2, b3, b4, b5))
        }
        #[inline]
        fn read_bools_6(&mut self) -> Result<(bool, bool, bool, bool, bool, bool)> {
            let b = self.read()?;
            if b > 0b111111 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b000001 != 0;
            let b2 = b & 0b000010 != 0;
            let b3 = b & 0b000100 != 0;
            let b4 = b & 0b001000 != 0;
            let b5 = b & 0b010000 != 0;
            let b6 = b & 0b100000 != 0;
            Ok((b1, b2, b3, b4, b5, b6))
        }
        #[inline]
        fn read_bools_7(&mut self) -> Result<(bool, bool, bool, bool, bool, bool, bool)> {
            let b = self.read()?;
            if b > 0b1111111 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid bools."));
            }
            let b1 = b & 0b0000001 != 0;
            let b2 = b & 0b0000010 != 0;
            let b3 = b & 0b0000100 != 0;
            let b4 = b & 0b0001000 != 0;
            let b5 = b & 0b0010000 != 0;
            let b6 = b & 0b0100000 != 0;
            let b7 = b & 0b1000000 != 0;
            Ok((b1, b2, b3, b4, b5, b6, b7))
        }
        #[inline]
        fn read_bools_8(&mut self) -> Result<(bool, bool, bool, bool, bool, bool, bool, bool)> {
            let b = self.read()?;
            let b1 = b & 0b00000001 != 0;
            let b2 = b & 0b00000010 != 0;
            let b3 = b & 0b00000100 != 0;
            let b4 = b & 0b00001000 != 0;
            let b5 = b & 0b00010000 != 0;
            let b6 = b & 0b00100000 != 0;
            let b7 = b & 0b01000000 != 0;
            let b8 = b & 0b10000000 != 0;
            Ok((b1, b2, b3, b4, b5, b6, b7, b8))
        }
    };
}
pub(crate) use define_bools_read;

macro_rules! define_bools_write {
    () => {
        #[inline]
        fn write_bools_2(&mut self, b1: bool, b2: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b01; }
            if b2 { b |= 0b10; }
            self.write(b)
        }
        #[inline]
        fn write_bools_3(&mut self, b1: bool, b2: bool, b3: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b001; }
            if b2 { b |= 0b010; }
            if b3 { b |= 0b100; }
            self.write(b)
        }
        #[inline]
        fn write_bools_4(&mut self, b1: bool, b2: bool, b3: bool, b4: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b0001; }
            if b2 { b |= 0b0010; }
            if b3 { b |= 0b0100; }
            if b4 { b |= 0b1000; }
            self.write(b)
        }
        #[inline]
        fn write_bools_5(&mut self, b1: bool, b2: bool, b3: bool, b4: bool, b5: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b00001; }
            if b2 { b |= 0b00010; }
            if b3 { b |= 0b00100; }
            if b4 { b |= 0b01000; }
            if b5 { b |= 0b10000; }
            self.write(b)
        }
        #[inline]
        fn write_bools_6(&mut self, b1: bool, b2: bool, b3: bool, b4: bool, b5: bool, b6: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b000001; }
            if b2 { b |= 0b000010; }
            if b3 { b |= 0b000100; }
            if b4 { b |= 0b001000; }
            if b5 { b |= 0b010000; }
            if b6 { b |= 0b100000; }
            self.write(b)
        }
        #[inline]
        fn write_bools_7(&mut self, b1: bool, b2: bool, b3: bool, b4: bool, b5: bool, b6: bool, b7: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b0000001; }
            if b2 { b |= 0b0000010; }
            if b3 { b |= 0b0000100; }
            if b4 { b |= 0b0001000; }
            if b5 { b |= 0b0010000; }
            if b6 { b |= 0b0100000; }
            if b7 { b |= 0b1000000; }
            self.write(b)
        }
        #[inline]
        fn write_bools_8(&mut self, b1: bool, b2: bool, b3: bool, b4: bool, b5: bool, b6: bool, b7: bool, b8: bool) -> Result<usize> {
            let mut b = 0;
            if b1 { b |= 0b00000001; }
            if b2 { b |= 0b00000010; }
            if b3 { b |= 0b00000100; }
            if b4 { b |= 0b00001000; }
            if b5 { b |= 0b00010000; }
            if b6 { b |= 0b00100000; }
            if b7 { b |= 0b01000000; }
            if b8 { b |= 0b10000000; }
            self.write(b)
        }
    };
}
pub(crate) use define_bools_write;

#[cfg(test)]
mod codegen {
    use std::io::{Cursor, stdout, Write};

    #[test]
    fn read() {
        let mut buf = Cursor::new(Vec::new());
        for i in 2..=8 {
            writeln!(buf, "        #[inline]").unwrap();
            write!(buf, "        fn read_bools_{i}(&mut self) -> Result<(bool").unwrap();
            for _ in 1..i {
                write!(buf, ", bool").unwrap();
            }
            writeln!(buf, ")> {{").unwrap();
            writeln!(buf, "            let b = self.read()?;").unwrap();
            if i < 8 {
                write!(buf, "            if b > 0b").unwrap();
                for _ in 0..i {
                    write!(buf, "1").unwrap();
                }
                writeln!(buf, " {{").unwrap();
                writeln!(buf, "                return Err(Error::new(ErrorKind::InvalidData, \"Invalid bools.\"));").unwrap();
                writeln!(buf, "            }}").unwrap();
            }
            for j in 1..=i {
                write!(buf, "            let b{j} = b & 0b").unwrap();
                for k in 0..i {
                    write!(buf, "{}", if k == i - j { 1 } else { 0 }).unwrap();
                }
                writeln!(buf, " != 0;").unwrap();
            }
            write!(buf, "            Ok((b1").unwrap();
            for j in 2..=i {
                write!(buf, ", b{j}").unwrap();
            }
            writeln!(buf, "))").unwrap();
            writeln!(buf, "        }}").unwrap();
        }
        stdout().write_all(buf.get_ref()).unwrap();
    }

    #[test]
    fn write() {
        let mut buf = Cursor::new(Vec::new());
        for i in 2..=8 {
            writeln!(buf, "        #[inline]").unwrap();
            write!(buf, "        fn write_bools_{i}(&mut self").unwrap();
            for j in 1..=i {
                write!(buf, ", b{j}: bool").unwrap();
            }
            writeln!(buf, ") -> Result<usize> {{").unwrap();
            writeln!(buf, "            let mut b = 0;").unwrap();
            for j in 1..=i {
                write!(buf, "            if b{j} {{ b |= 0b").unwrap();
                for k in 0..i {
                    write!(buf, "{}", if k == i - j { 1 } else { 0 }).unwrap();
                }
                writeln!(buf, "; }}").unwrap();
            }
            writeln!(buf, "            self.write(b)").unwrap();
            writeln!(buf, "        }}").unwrap();
        }
        stdout().write_all(buf.get_ref()).unwrap();
    }
}
