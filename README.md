
# Table of Contents

-   [About](#org2364844)
-   [Benchmark](#org0ef8815)
-   [Examples](#org813bcff)
    -   [Wrap multiple languages](#orged47e8f)
        -   [Space-sensitive (English, French, Spanish, etc.)](#org8c70dc1)
        -   [Space-insensitive (Chinese, Japanese, Thai, etc.)](#orgb6bb815)
    -   [Wrap and append/prepend](#org1c32e9d)
        -   [Indentation](#org376dbae)
        -   [Trailing notation](#orgc454678)



<a id="org2364844"></a>

# About

Bwrap is a lightweight, embedded environment-friendly library for
wrapping text. While Bwrap offers great flexibility in wrapping text,
neither performance nor resource consumption compromises:

1.  The time/space complexity is *O(n)* by default, or *O(n(p+a))* if
    there is appending/prepending. (*n*, *p*, *a* is the number of
    input bytes, prepended bytes, and appended bytes respectively)

2.  No heap allocation happens by default.

For the sake of readability, we (**b**)etter **wrap** our text.


<a id="org0ef8815"></a>

# Benchmark

Here several text-wrapping libraries(including bwrap) are benchmarked
in two dimensions:

**Time elapsed:**

<img src="result-rtime.png" width="500"/>

**Memory usage:**

<img src="result-mempeak.png" width="500"/>

Notes:

1.  Samples are generated from a small paragraph(3085 Bytes) excerpted
    from GPLv3 license, resulting in files of size ranging from 3 MB to  
    100 MB.

2.  All libraries are assigned the same job: take samples from STDIN,
    wrap them with an 80-width limit, then print out the result to
    STDOUT.

3.  Although not all libraries' results are identical, they are
    correct enough, i.e. no lines exceed the 80-width limit.

Note: Details about benchmark samples or methods are in
[bench-wrap-libs](https://github.com/imichael2e2/bench-wrap-libs).


<a id="org813bcff"></a>

# Examples


<a id="orged47e8f"></a>

## Wrap multiple languages

Bwrap categorizes languages into two categories, the first is for the
languages that depend on ASCII SPACE to delimit words, such as
English, French, Italian, Spanish and so on. The second is for the
languages that are space-insensitive, such as Chinese, Japanese, Thai
and so on.


<a id="org8c70dc1"></a>

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


<a id="orgb6bb815"></a>

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


<a id="org1c32e9d"></a>

## Wrap and append/prepend

Bwrap can append or prepend whatever string to newly added newline
character. With this feature, one can effectively achieve indentation,
line trailing notation or similar.


<a id="org376dbae"></a>

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


<a id="orgc454678"></a>

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

