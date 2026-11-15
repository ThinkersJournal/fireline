// SPDX-License-Identifier: MIT OR Apache-2.0
//! A minimal JSON reader: just enough for KISS's generated sidecars (`op_facts.json`, `dtype_manifest.json`), kept private so the crate stays
//! dependency-free. Duplicate object keys are refused; nesting is capped.

pub(crate) enum Value {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

impl Value {
    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub(crate) fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Num(s) => s.parse().ok(),
            _ => None,
        }
    }
    pub(crate) fn as_arr(&self) -> Option<&[Value]> {
        match self {
            Value::Arr(a) => Some(a),
            _ => None,
        }
    }
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
}

pub(crate) fn parse(text: &str) -> Result<Value, usize> {
    let mut p = P {
        b: text.as_bytes(),
        i: 0,
        depth: 0,
    };
    let v = p.value()?;
    p.ws();
    if p.i == p.b.len() { Ok(v) } else { Err(p.i) }
}

struct P<'a> {
    b: &'a [u8],
    i: usize,
    depth: usize,
}

impl P<'_> {
    fn ws(&mut self) {
        while matches!(self.b.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }
    fn lit(&mut self, word: &str, v: Value) -> Result<Value, usize> {
        if self.b[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(self.i)
        }
    }
    fn value(&mut self) -> Result<Value, usize> {
        self.ws();
        match self.b.get(self.i).copied() {
            Some(b'n') => self.lit("null", Value::Null),
            Some(b't') => self.lit("true", Value::Bool(true)),
            Some(b'f') => self.lit("false", Value::Bool(false)),
            Some(b'"') => self.string().map(Value::Str),
            Some(b'[') => self.nest(|p| p.array()),
            Some(b'{') => self.nest(|p| p.object()),
            Some(b'-' | b'0'..=b'9') => {
                let s = self.i;
                while matches!(
                    self.b.get(self.i),
                    Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                ) {
                    self.i += 1;
                }
                if self.i == s {
                    Err(s)
                } else {
                    Ok(Value::Num(
                        String::from_utf8_lossy(&self.b[s..self.i]).into_owned(),
                    ))
                }
            }
            _ => Err(self.i),
        }
    }
    fn nest(&mut self, f: fn(&mut Self) -> Result<Value, usize>) -> Result<Value, usize> {
        self.depth += 1;
        if self.depth > 64 {
            return Err(self.i);
        }
        let v = f(self);
        self.depth -= 1;
        v
    }
    fn array(&mut self) -> Result<Value, usize> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b']') {
            self.i += 1;
            return Ok(Value::Arr(out));
        }
        loop {
            out.push(self.value()?);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Value::Arr(out));
                }
                _ => return Err(self.i),
            }
        }
    }
    fn object(&mut self) -> Result<Value, usize> {
        self.i += 1;
        let mut out: Vec<(String, Value)> = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Ok(Value::Obj(out));
        }
        loop {
            self.ws();
            if self.b.get(self.i) != Some(&b'"') {
                return Err(self.i);
            }
            let at = self.i;
            let k = self.string()?;
            if out.iter().any(|(e, _)| *e == k) {
                return Err(at);
            }
            self.ws();
            if self.b.get(self.i) != Some(&b':') {
                return Err(self.i);
            }
            self.i += 1;
            out.push((k, self.value()?));
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Value::Obj(out));
                }
                _ => return Err(self.i),
            }
        }
    }
    fn string(&mut self) -> Result<String, usize> {
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or(self.i)?;
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).map_err(|_| self.i),
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or(self.i)?;
                    self.i += 1;
                    match e {
                        b'"' | b'\\' | b'/' => out.push(e),
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let h = self.b.get(self.i..self.i + 4).ok_or(self.i)?;
                            let h = core::str::from_utf8(h).map_err(|_| self.i)?;
                            let n = u32::from_str_radix(h, 16).map_err(|_| self.i)?;
                            let ch = char::from_u32(n).ok_or(self.i)?;
                            self.i += 4;
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return Err(self.i),
                    }
                }
                0..=31 => return Err(self.i),
                _ => out.push(c),
            }
        }
    }
}
