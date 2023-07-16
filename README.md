
# Table of Contents

-   [About](#orga96a3de)
-   [Benchmark](#org94ca2fe)
-   [Examples](#orgd41df4b)
    -   [Wrap multiple languages](#org39c1bf3)
        -   [Space-sensitive (English, French, Spanish, etc.)](#orgc0b5aba)
        -   [Space-insensitive (Chinese, Japanese, Thai, etc.)](#orgc58b65a)
    -   [Wrap and append/prepend](#org33a1643)
        -   [Indentation](#orgfdf4fd4)
        -   [Trailing notation](#org33be651)



<a id="orga96a3de"></a>

# About

Bwrap is a lightweight, embedded environment-friendly library for
wrapping text. While Bwrap offers great flexibility in wrapping text,
neither performance nor resource consumption compromises:

1.  The time/space complexity is *O(n)* by default, or *O(n(p+a))* if
    there is appending/prepending. (*n*, *p*, *a* is the number of
    input bytes, prepended bytes, and appended bytes respectively)

2.  No heap allocation happens by default.

For the sake of readability, we (**b**)etter **wrap** our text.


<a id="org94ca2fe"></a>

# Benchmark

The below is the performance comparison among several text-wrapping
libraries, including Bwrap. Details about benchmark samples or methods
are elaborated in [bench-wrap-libs](https://github.com/imichael2e2/bench-wrap-libs).

**Time elapsed:**

<img src="result-rtime.png" width="500"/>

**Memory usage:**

<img src="result-mempeak.png" width="500"/>


<a id="orgd41df4b"></a>

# Examples


<a id="org39c1bf3"></a>

## Wrap multiple languages

Bwrap categorizes languages into two categories, the first is for the
languages that depend on ASCII SPACE to delimit words, such as
English, French, Italian, Spanish and so on. The second is for the
languages that are space-insensitive, such as Chinese, Japanese, Thai
and so on.


<a id="orgc0b5aba"></a>

### Space-sensitive (English, French, Spanish, etc.)

The languages that rely on SPACE(ASCII SPACE).

-   English

        
        ORIGINAL:
        
        one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten
        
        WRAPPED:
        
        one two three four five six seven eight nine ten
        one two three four five six seven eight nine ten
        one two three four five six seven eight nine ten
    
    Code:
    
        use bwrap::EasyWrapper;
        
        let line = "one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten";
        println!("ORIGINAL:\n\n{}\n", line);
        
        let mut w = EasyWrapper::new(line, 50).unwrap();
        let wrapped = w.wrap().unwrap();
        println!("WRAPPED:\n\n{}", wrapped);

-   French

    TODO

-   Spanish

    TODO


<a id="orgc58b65a"></a>

### Space-insensitive (Chinese, Japanese, Thai, etc.)

The languages that does not rely on ASCII SPACE.

-   Chinese

        
        ORIGINAL:
        
        一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十
        
        WRAPPED:
        
        一二三四五六七八九十
        一二三四五六七八九十
        一二三四五六七八九十
    
    Code: 
    
        
        use bwrap::{EasyWrapper, WrapStyle::MayBrk};
        
        let line = "一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十";
        println!("ORIGINAL:\n\n{}\n", line);
        
        let mut w = EasyWrapper::new(line, 20).unwrap();
        let wrapped = w.wrap_use_style(MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);

-   Japanese

        
        ORIGINAL:
        
        ありがとうございますありがとうございますありがとうございます
        
        WRAPPED:
        
        ありがとうございます
        ありがとうございます
        ありがとうございます
    
    Code:
    
        
        use bwrap::{EasyWrapper, WrapStyle::MayBrk};
        
        let line = "ありがとうございますありがとうございますありがとうございます";
        println!("ORIGINAL:\n\n{}\n", line);
        
        let mut w = EasyWrapper::new(line, 10).unwrap();
        let wrapped = w.wrap_use_style(MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);

-   Thai

        
        ORIGINAL:
        
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
        
        WRAPPED:
        
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    
    Code: 
    
        
        use bwrap::{EasyWrapper, WrapStyle::MayBrk};
        
        let line = "หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ";
        println!("ORIGINAL:\n\n{}\n", line);
        
        let mut w = EasyWrapper::new(line, 25).unwrap();
        let wrapped = w.wrap_use_style(MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);


<a id="org33a1643"></a>

## Wrap and append/prepend

Bwrap can append or prepend whatever string to newly added newline
character. With this feature, one can effectively achieve indentation,
line trailing notation or similar.


<a id="orgfdf4fd4"></a>

### Indentation

    
    ORIGINAL:
    
    Here is our schedule:
    - Do A, and do B, and do C, and do D, and do E, and do F
    - Do G, and do H, and do I, and do J, and do K, and do L
    
    WRAPPED:
    
    Here is our schedule:
    - Do A, and do B, and do C, and do
      D, and do E, and do F
    - Do G, and do H, and do I, and do
      J, and do K, and do L

Code:

    
    use bwrap::{EasyWrapper, ExistNlPref, WrapStyle::NoBrk};
    
    let line = "Here is our schedule:\n- Do A, and do B, and do C, and do D, and do E, and do F\n- Do G, and\
    	    do H, and do I, and do J, and do K, and do L";
    println!("ORIGINAL:\n\n{}\n", line);
    
    let mut w = EasyWrapper::new(line, 35).unwrap();
    let wrapped = w.wrap_use_style(NoBrk(Some("  "), ExistNlPref::KeepTrailSpc))
        .unwrap();
    println!("WRAPPED:\n\n{}", wrapped);


<a id="org33be651"></a>

### Trailing notation

    
    ORIGINAL:
    
    VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK
    
    WRAPPED:
    
    VGhpcyBpcy |
    BhIHNlY3Jl |
    dCBtZXNzYW |
    dlLCBwbGVh |
    c2UgZGVsZX |
    RlIGFmdGVy |
    IHJlYWQK

Code:

    
    use bwrap::{EasyWrapper, WrapStyle::MayBrk};
    
    let line = "VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK";
    println!("ORIGINAL:\n\n{}\n", line);
    
    let mut w = EasyWrapper::new(line, 10).unwrap();
    let wrapped = w.wrap_use_style(MayBrk(Some(" |"), None)).unwrap();
    println!("WRAPPED:\n\n{}", wrapped);

