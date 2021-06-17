pub mod recursive_descent;
pub mod recursive_descent_2;

pub trait Solution {
    fn is_valid(code: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("<DIV>This is the first line <![CDATA[<div>]]></DIV>", true),
            ("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>", true),
            ("<A>  <B> </A>   </B>", false),
            ("<DIV>  div tag is not closed  <DIV>", false),
            (
                "<DIV>\
                     <YFSYYS>\
                         <UVBNIQ>\
                             <XPMXUNT>\
                                 <WNGMV>\
                                     <OJJGQREMT>\
                                         <Z>\
                                             <GEJDP>\
                                                 <LIQS>\
                                                     <NCVYU>\
                                                         <RAS>\
                                                             <UYFKCJCDN>\
                                                                 <NA>\
                                                                     <POJVYT>\
                                                                         <Z>\
                                                                             <TDC>\
                                                                                 <VUIZQC>\
                                                                                     <BNANGX>\
                                                                                         <TOF>\
                                                                                             <MR>MK</MR>\
                                                                                         </TOF>\
                                                                                     </BNANGX>\
                                                                                 </VUIZQC>\
                                                                             </TDC>\
                                                                         </Z>\
                                                                     </POJVYT>\
                                                                 </NA>\
                                                             </UYFKCJCDN>\
                                                         </RAS>\
                                                     </NCVYU>\
                                                 </LIQS>\
                                             </GEJDP>\
                                         </Z>\
                                     </OJJGQREMT>\
                                 </WNGMV>\
                             </XPMXUNT>\
                         </UVBNIQ>\
                     </YFSYYS>\
                 </DIV>",
                true,
            ),
        ];

        for (code, expected) in test_cases {
            assert_eq!(S::is_valid(code.to_string()), expected);
        }
    }
}
