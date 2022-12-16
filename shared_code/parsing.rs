use crate::error::{dyn_parse_error, Error, parse_error};

#[allow(dead_code)]
fn parse_prefix<'a>(s:&'a str,prefix:&str)->Result<&'a str,Error>{
    if let Some(rem) = s.strip_prefix(prefix){
        Ok(rem)
    } else {
        Err(dyn_parse_error(format!("Expected: '{}', found: '{}'",prefix,s)))
    }
}

#[allow(dead_code)]
fn parse_prefix_maybe<'a>(s:&'a str,prefix:&str)->Result<&'a str,Error>{
    if let Ok(rem) = parse_prefix(s,prefix){
        Ok(rem)
    } else {
        Ok(s)
    }
}

#[allow(dead_code)]
fn parse_num(mut s:&str)->Result<(&str,i32),Error>{
    let mut is_neg = false;
    if let Ok(rem) = parse_prefix(s,"-"){
        is_neg = true;
        s = rem;
    }
    let i = s.find(|c:char|!c.is_ascii_digit()).unwrap_or(s.len());
    let mut val:i32 = s[0..i].parse()?;
    if is_neg{
        val= val*-1;
    }
    Ok((&s[i..],val))
}

fn parse_until<'a>(s:&'a str,delimiter:&str)->Result<(&'a str,&'a str),Error>{
    let i = s.find(delimiter).unwrap_or(s.len());
        //.ok_or(dyn_parse_error(format!("Expected: '{}'",delimiter)))?;
    Ok((&s[i..],&s[0..i]))
}

///Parses parses a string with repeated occurances of a delimiter into a vecotor of the pieces occurring between the delimitor
fn parse_list<'a,T,F:Fn(&'a str)->Result<(&'a str,T),Error>>(mut s:&'a str, delimiter:&str,parser:F) ->Result<(&'a str, Vec<T>),Error>{
    let mut results:Vec<T> = Vec::new();
    loop {
        let (rem,val) = parser(s)?;
        results.push(val);
        if let Some(rem2) = rem.strip_prefix(delimiter){
            s = rem2;
        } else {
            return Ok((rem,results));
        }
    }
}


pub trait Parsable<'a> {
    type NumResType;
    type StringResType;
    type NoResType;
    type VecResType<T>;
    fn parse_prefix(self, prefix:&str) ->Result<Self::NoResType, Error>;
    fn parse_num(self)->Result<(&'a str,Self::NumResType),Error>;
    fn parse_id(self,delimiter:&str)->Result<(&'a str,Self::StringResType),Error>;
    fn parse_lst<T,F:Fn(&'a str)->Result<(&'a str,T),Error>>(self,delimiter:&str,parser:F)->Result<(&'a str, Self::VecResType<T>),Error>;
    fn parse_maybe(self,prefix:&str)->Result<Self::NoResType,Error>;

}

impl <'a> Parsable<'a> for &'a str {
    type NumResType = i32;
    type StringResType = &'a str;
    type NoResType = &'a str;
    type VecResType<U> = Vec<U>;

    fn parse_prefix(self, prefix: &str) -> Result<Self::NoResType, Error> {
        parse_prefix(self,prefix)
    }

    fn parse_num(self) -> Result<(&'a str, Self::NumResType), Error> {
        parse_num(self)
    }

    fn parse_id(self,delimiter:&str) -> Result<(&'a str, Self::StringResType), Error> {
        parse_until(self,delimiter)
    }

    fn parse_lst<P, F: Fn(&'a str) -> Result<(&'a str, P), Error>>(self, delimiter: &str, parser: F)->Result<(&'a str, Self::VecResType<P>),Error> {
        parse_list(self,delimiter,parser)
    }

    fn parse_maybe(self, prefix: &str) -> Result<Self::NoResType, Error> {
        parse_prefix_maybe(self,prefix)
    }
}

pub trait WrapsParsable<'a>{
    type InnerParsType:Parsable<'a>;
    type RemainderType;
    type NoResType;
    type ContainerType<X>;
    fn get_parsable(self)->Result<(Self::InnerParsType,Self::RemainderType),Error>;
    fn build_no_type(val:Self::RemainderType,x:<Self::InnerParsType as Parsable<'a>>::NoResType)->Self::NoResType;
    fn build_container_type<U>(val:Self::RemainderType,input:U)->Self::ContainerType<U>;
}

impl <'a,T> WrapsParsable<'a> for (&'a str, T) {
    type InnerParsType = &'a str;
    type RemainderType = T;
    type NoResType = (&'a str,T);
    type ContainerType<X> = (T,X);

    fn get_parsable(self) -> Result<(Self::InnerParsType,Self::RemainderType),Error> {
        Ok(self)
    }

    fn build_no_type(val:Self::RemainderType, x: <Self::InnerParsType as Parsable<'a>>::NoResType) -> Self::NoResType {
        (x,val)
    }

    fn build_container_type<U>(val:Self::RemainderType, input: U) -> Self::ContainerType<U> {
        (val,input)
    }
}

impl <'a,T> Parsable<'a> for T
where T:WrapsParsable<'a>{
    type NumResType = T::ContainerType<<T::InnerParsType as Parsable<'a>>::NumResType>;
    type StringResType = T::ContainerType<<T::InnerParsType as Parsable<'a>>::StringResType>;
    type NoResType = T::NoResType;
    type VecResType<U> = T::ContainerType<<T::InnerParsType as Parsable<'a>>::VecResType<U>>;

    fn parse_prefix(self, prefix: &str) -> Result<Self::NoResType, Error> {
        let (parser,val) = self.get_parsable()?;
        parser.parse_prefix(prefix).map(|res|T::build_no_type(val,res))
    }

    fn parse_num(self) -> Result<(&'a str, Self::NumResType), Error> {
        let (parser,val) = self.get_parsable()?;
        parser.parse_num().map(|(rem,res)|(rem,T::build_container_type(val,res)))

    }

    fn parse_id(self, delimiter: &str) -> Result<(&'a str, Self::StringResType), Error> {
        let (parser,val) = self.get_parsable()?;
        parser.parse_id(delimiter).map(|(rem,res)|(rem,T::build_container_type(val,res)))

    }

    fn parse_lst<V, F: Fn(&'a str) -> Result<(&'a str, V), Error>>(self, delimiter: &str, inner_parser: F) -> Result<(&'a str, Self::VecResType<V>), Error> {
        let (parser,val) = self.get_parsable()?;
        parser.parse_lst(delimiter,inner_parser).map(|(rem,res)|(rem,T::build_container_type(val,res)))

    }

    fn parse_maybe(self, prefix: &str) -> Result<Self::NoResType, Error> {
        let (parser,val) = self.get_parsable()?;
        parser.parse_maybe(prefix).map(|res|T::build_no_type(val,res))
    }
}

impl < 'a, T:Parsable<'a>> WrapsParsable<'a> for Result<T,Error> {
    type InnerParsType = T;
    type RemainderType = ();
    type NoResType = T::NoResType;
    type ContainerType<X> = X;

    fn get_parsable(self) -> Result<(Self::InnerParsType,Self::RemainderType),Error> {
        self.map(|v|(v,()))
    }

    fn build_no_type(_: Self::RemainderType, x: <Self::InnerParsType as Parsable<'a>>::NoResType) -> Self::NoResType {
        x
    }

    fn build_container_type<U>(_: Self::RemainderType, input: U) -> Self::ContainerType<U> {
        input
    }
}
