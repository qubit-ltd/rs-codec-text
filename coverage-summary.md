## Code coverage

Historical coverage snapshot. The percentages and annotated source below do not describe the current HEAD. The original generation date, source commit, toolchain, command, and feature configuration were not recorded. For current coverage, consult the [published CI coverage report](https://qubit-ltd.github.io/rs-codec-text/coverage/) and verify the associated CI run and source revision. Availability of a published report does not establish that it covers the current HEAD.

| Metric | Covered | Total | Coverage |
| --- | ---: | ---: | ---: |
| functions | 401 | 401 | 100% |
| lines | 3297 | 3315 | 99.46% |
| regions | 4213 | 4248 | 99.18% |

- HTML report: artifact `coverage-reports`, path `target/llvm-cov/html/index.html`.
- LCOV report: artifact `coverage-reports`, path `lcov.info`.
- Shields endpoint: artifact `coverage-reports`, path `coverage-badge.json`.

<details><summary>Text summary</summary>

```text
/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/ascii.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use crate::charset::ascii_folding;
    9|       |
   10|       |/// Namespace for ASCII character and code point helpers.
   11|       |pub enum Ascii {}
   12|       |
   13|       |impl Ascii {
   14|       |    /// Maximum valid ASCII character.
   15|       |    pub const MAX_CHAR: char = '\u{007f}';
   16|       |
   17|       |    /// Maximum valid ASCII byte.
   18|       |    pub const MAX_BYTE: u8 = Self::MAX_CHAR as u8;
   19|       |
   20|       |    /// Maximum number of ASCII characters emitted by [`Self::fold`].
   21|       |    pub const MAX_FOLDING_COUNT: usize = 4;
   22|       |
   23|       |    /// ASCII NUL.
   24|       |    pub const NULL_CHAR: char = '\0';
   25|       |
   26|       |    /// ASCII NUL as a byte.
   27|       |    pub const NULL_BYTE: u8 = Self::NULL_CHAR as u8;
   28|       |
   29|       |    /// ASCII SOH.
   30|       |    pub const START_OF_HEADER_CHAR: char = '\u{0001}';
   31|       |
   32|       |    /// ASCII SOH as a byte.
   33|       |    pub const START_OF_HEADER_BYTE: u8 = Self::START_OF_HEADER_CHAR as u8;
   34|       |
   35|       |    /// ASCII STX.
   36|       |    pub const START_OF_TEXT_CHAR: char = '\u{0002}';
   37|       |
   38|       |    /// ASCII STX as a byte.
   39|       |    pub const START_OF_TEXT_BYTE: u8 = Self::START_OF_TEXT_CHAR as u8;
   40|       |
   41|       |    /// ASCII ETX.
   42|       |    pub const END_OF_TEXT_CHAR: char = '\u{0003}';
   43|       |
   44|       |    /// ASCII ETX as a byte.
   45|       |    pub const END_OF_TEXT_BYTE: u8 = Self::END_OF_TEXT_CHAR as u8;
   46|       |
   47|       |    /// ASCII EOT.
   48|       |    pub const START_OF_TRANSMISSION_CHAR: char = '\u{0004}';
   49|       |
   50|       |    /// ASCII EOT as a byte.
   51|       |    pub const START_OF_TRANSMISSION_BYTE: u8 = Self::START_OF_TRANSMISSION_CHAR as u8;
   52|       |
   53|       |    /// ASCII ENQ.
   54|       |    pub const ENQUIRY_CHAR: char = '\u{0005}';
   55|       |
   56|       |    /// ASCII ENQ as a byte.
   57|       |    pub const ENQUIRY_BYTE: u8 = Self::ENQUIRY_CHAR as u8;
   58|       |
   59|       |    /// ASCII ACK.
   60|       |    pub const ACKNOWLEDGMENT_CHAR: char = '\u{0006}';
   61|       |
   62|       |    /// ASCII ACK as a byte.
   63|       |    pub const ACKNOWLEDGMENT_BYTE: u8 = Self::ACKNOWLEDGMENT_CHAR as u8;
   64|       |
   65|       |    /// ASCII BEL.
   66|       |    pub const BELL_CHAR: char = '\u{0007}';
   67|       |
   68|       |    /// ASCII BEL as a byte.
   69|       |    pub const BELL_BYTE: u8 = Self::BELL_CHAR as u8;
   70|       |
   71|       |    /// ASCII BS.
   72|       |    pub const BACKSPACE_CHAR: char = '\u{0008}';
   73|       |
   74|       |    /// ASCII BS as a byte.
   75|       |    pub const BACKSPACE_BYTE: u8 = Self::BACKSPACE_CHAR as u8;
   76|       |
   77|       |    /// ASCII HT.
   78|       |    pub const HORIZONTAL_TAB_CHAR: char = '\t';
   79|       |
   80|       |    /// ASCII HT as a byte.
   81|       |    pub const HORIZONTAL_TAB_BYTE: u8 = Self::HORIZONTAL_TAB_CHAR as u8;
   82|       |
   83|       |    /// ASCII LF.
   84|       |    pub const LINE_FEED_CHAR: char = '\n';
   85|       |
   86|       |    /// ASCII LF as a byte.
   87|       |    pub const LINE_FEED_BYTE: u8 = Self::LINE_FEED_CHAR as u8;
   88|       |
   89|       |    /// ASCII VT.
   90|       |    pub const VERTICAL_TAB_CHAR: char = '\u{000b}';
   91|       |
   92|       |    /// ASCII VT as a byte.
   93|       |    pub const VERTICAL_TAB_BYTE: u8 = Self::VERTICAL_TAB_CHAR as u8;
   94|       |
   95|       |    /// ASCII FF.
   96|       |    pub const FORM_FEED_CHAR: char = '\u{000c}';
   97|       |
   98|       |    /// ASCII FF as a byte.
   99|       |    pub const FORM_FEED_BYTE: u8 = Self::FORM_FEED_CHAR as u8;
  100|       |
  101|       |    /// ASCII CR.
  102|       |    pub const CARRIAGE_RETURN_CHAR: char = '\r';
  103|       |
  104|       |    /// ASCII CR as a byte.
  105|       |    pub const CARRIAGE_RETURN_BYTE: u8 = Self::CARRIAGE_RETURN_CHAR as u8;
  106|       |
  107|       |    /// ASCII SO.
  108|       |    pub const SHIFT_OUT_CHAR: char = '\u{000e}';
  109|       |
  110|       |    /// ASCII SO as a byte.
  111|       |    pub const SHIFT_OUT_BYTE: u8 = Self::SHIFT_OUT_CHAR as u8;
  112|       |
  113|       |    /// ASCII SI.
  114|       |    pub const SHIFT_IN_CHAR: char = '\u{000f}';
  115|       |
  116|       |    /// ASCII SI as a byte.
  117|       |    pub const SHIFT_IN_BYTE: u8 = Self::SHIFT_IN_CHAR as u8;
  118|       |
  119|       |    /// ASCII DLE.
  120|       |    pub const DATA_LINK_ESCAPE_CHAR: char = '\u{0010}';
  121|       |
  122|       |    /// ASCII DLE as a byte.
  123|       |    pub const DATA_LINK_ESCAPE_BYTE: u8 = Self::DATA_LINK_ESCAPE_CHAR as u8;
  124|       |
  125|       |    /// ASCII DC1.
  126|       |    pub const DEVICE_CONTROL_1_CHAR: char = '\u{0011}';
  127|       |
  128|       |    /// ASCII DC1 as a byte.
  129|       |    pub const DEVICE_CONTROL_1_BYTE: u8 = Self::DEVICE_CONTROL_1_CHAR as u8;
  130|       |
  131|       |    /// ASCII DC2.
  132|       |    pub const DEVICE_CONTROL_2_CHAR: char = '\u{0012}';
  133|       |
  134|       |    /// ASCII DC2 as a byte.
  135|       |    pub const DEVICE_CONTROL_2_BYTE: u8 = Self::DEVICE_CONTROL_2_CHAR as u8;
  136|       |
  137|       |    /// ASCII DC3.
  138|       |    pub const DEVICE_CONTROL_3_CHAR: char = '\u{0013}';
  139|       |
  140|       |    /// ASCII DC3 as a byte.
  141|       |    pub const DEVICE_CONTROL_3_BYTE: u8 = Self::DEVICE_CONTROL_3_CHAR as u8;
  142|       |
  143|       |    /// ASCII DC4.
  144|       |    pub const DEVICE_CONTROL_4_CHAR: char = '\u{0014}';
  145|       |
  146|       |    /// ASCII DC4 as a byte.
  147|       |    pub const DEVICE_CONTROL_4_BYTE: u8 = Self::DEVICE_CONTROL_4_CHAR as u8;
  148|       |
  149|       |    /// ASCII NAK.
  150|       |    pub const NEGATIVE_ACKNOWLEDGEMENT_CHAR: char = '\u{0015}';
  151|       |
  152|       |    /// ASCII NAK as a byte.
  153|       |    pub const NEGATIVE_ACKNOWLEDGEMENT_BYTE: u8 = Self::NEGATIVE_ACKNOWLEDGEMENT_CHAR as u8;
  154|       |
  155|       |    /// ASCII SYN.
  156|       |    pub const SYNCHRONOUS_IDLE_CHAR: char = '\u{0016}';
  157|       |
  158|       |    /// ASCII SYN as a byte.
  159|       |    pub const SYNCHRONOUS_IDLE_BYTE: u8 = Self::SYNCHRONOUS_IDLE_CHAR as u8;
  160|       |
  161|       |    /// ASCII ETB.
  162|       |    pub const END_OF_TRANS_BLOCK_CHAR: char = '\u{0017}';
  163|       |
  164|       |    /// ASCII ETB as a byte.
  165|       |    pub const END_OF_TRANS_BLOCK_BYTE: u8 = Self::END_OF_TRANS_BLOCK_CHAR as u8;
  166|       |
  167|       |    /// ASCII CAN.
  168|       |    pub const CANCEL_CHAR: char = '\u{0018}';
  169|       |
  170|       |    /// ASCII CAN as a byte.
  171|       |    pub const CANCEL_BYTE: u8 = Self::CANCEL_CHAR as u8;
  172|       |
  173|       |    /// ASCII EM.
  174|       |    pub const END_OF_MEDIUM_CHAR: char = '\u{0019}';
  175|       |
  176|       |    /// ASCII EM as a byte.
  177|       |    pub const END_OF_MEDIUM_BYTE: u8 = Self::END_OF_MEDIUM_CHAR as u8;
  178|       |
  179|       |    /// ASCII SUB.
  180|       |    pub const SUBSTITUTE_CHAR: char = '\u{001a}';
  181|       |
  182|       |    /// ASCII SUB as a byte.
  183|       |    pub const SUBSTITUTE_BYTE: u8 = Self::SUBSTITUTE_CHAR as u8;
  184|       |
  185|       |    /// ASCII ESC.
  186|       |    pub const ESCAPE_CHAR: char = '\u{001b}';
  187|       |
  188|       |    /// ASCII ESC as a byte.
  189|       |    pub const ESCAPE_BYTE: u8 = Self::ESCAPE_CHAR as u8;
  190|       |
  191|       |    /// ASCII FS.
  192|       |    pub const FILE_SEPARATOR_CHAR: char = '\u{001c}';
  193|       |
  194|       |    /// ASCII FS as a byte.
  195|       |    pub const FILE_SEPARATOR_BYTE: u8 = Self::FILE_SEPARATOR_CHAR as u8;
  196|       |
  197|       |    /// ASCII GS.
  198|       |    pub const GROUP_SEPARATOR_CHAR: char = '\u{001d}';
  199|       |
  200|       |    /// ASCII GS as a byte.
  201|       |    pub const GROUP_SEPARATOR_BYTE: u8 = Self::GROUP_SEPARATOR_CHAR as u8;
  202|       |
  203|       |    /// ASCII RS.
  204|       |    pub const RECORD_SEPARATOR_CHAR: char = '\u{001e}';
  205|       |
  206|       |    /// ASCII RS as a byte.
  207|       |    pub const RECORD_SEPARATOR_BYTE: u8 = Self::RECORD_SEPARATOR_CHAR as u8;
  208|       |
  209|       |    /// ASCII US.
  210|       |    pub const UNIT_SEPARATOR_CHAR: char = '\u{001f}';
  211|       |
  212|       |    /// ASCII US as a byte.
  213|       |    pub const UNIT_SEPARATOR_BYTE: u8 = Self::UNIT_SEPARATOR_CHAR as u8;
  214|       |
  215|       |    /// ASCII DEL.
  216|       |    pub const DELETE_CHAR: char = '\u{007f}';
  217|       |
  218|       |    /// ASCII DEL as a byte.
  219|       |    pub const DELETE_BYTE: u8 = Self::DELETE_CHAR as u8;
  220|       |
  221|       |    /// ASCII space.
  222|       |    pub const SPACE_CHAR: char = ' ';
  223|       |
  224|       |    /// ASCII space as a byte.
  225|       |    pub const SPACE_BYTE: u8 = Self::SPACE_CHAR as u8;
  226|       |
  227|       |    /// ASCII exclamation mark.
  228|       |    pub const EXCLAMATION_CHAR: char = '!';
  229|       |
  230|       |    /// ASCII exclamation mark as a byte.
  231|       |    pub const EXCLAMATION_BYTE: u8 = Self::EXCLAMATION_CHAR as u8;
  232|       |
  233|       |    /// ASCII double quote.
  234|       |    pub const DOUBLE_QUOTE_CHAR: char = '"';
  235|       |
  236|       |    /// ASCII double quote as a byte.
  237|       |    pub const DOUBLE_QUOTE_BYTE: u8 = Self::DOUBLE_QUOTE_CHAR as u8;
  238|       |
  239|       |    /// ASCII number sign.
  240|       |    pub const SHARP_CHAR: char = '#';
  241|       |
  242|       |    /// ASCII number sign as a byte.
  243|       |    pub const SHARP_BYTE: u8 = Self::SHARP_CHAR as u8;
  244|       |
  245|       |    /// ASCII dollar sign.
  246|       |    pub const DOLLAR_CHAR: char = '$';
  247|       |
  248|       |    /// ASCII dollar sign as a byte.
  249|       |    pub const DOLLAR_BYTE: u8 = Self::DOLLAR_CHAR as u8;
  250|       |
  251|       |    /// ASCII percent sign.
  252|       |    pub const PERCENT_CHAR: char = '%';
  253|       |
  254|       |    /// ASCII percent sign as a byte.
  255|       |    pub const PERCENT_BYTE: u8 = Self::PERCENT_CHAR as u8;
  256|       |
  257|       |    /// ASCII ampersand.
  258|       |    pub const AMPERSAND_CHAR: char = '&';
  259|       |
  260|       |    /// ASCII ampersand as a byte.
  261|       |    pub const AMPERSAND_BYTE: u8 = Self::AMPERSAND_CHAR as u8;
  262|       |
  263|       |    /// ASCII tab.
  264|       |    pub const TAB_CHAR: char = '\t';
  265|       |
  266|       |    /// ASCII tab as a byte.
  267|       |    pub const TAB_BYTE: u8 = Self::TAB_CHAR as u8;
  268|       |
  269|       |    /// ASCII backslash.
  270|       |    pub const BACKSLASH_CHAR: char = '\\';
  271|       |
  272|       |    /// ASCII backslash as a byte.
  273|       |    pub const BACKSLASH_BYTE: u8 = Self::BACKSLASH_CHAR as u8;
  274|       |
  275|       |    /// ASCII single quote.
  276|       |    pub const SINGLE_QUOTE_CHAR: char = '\'';
  277|       |
  278|       |    /// ASCII single quote as a byte.
  279|       |    pub const SINGLE_QUOTE_BYTE: u8 = Self::SINGLE_QUOTE_CHAR as u8;
  280|       |
  281|       |    /// ASCII back quote.
  282|       |    pub const BACK_QUOTE_CHAR: char = '`';
  283|       |
  284|       |    /// ASCII back quote as a byte.
  285|       |    pub const BACK_QUOTE_BYTE: u8 = Self::BACK_QUOTE_CHAR as u8;
  286|       |
  287|       |    /// ASCII comma.
  288|       |    pub const COMMA_CHAR: char = ',';
  289|       |
  290|       |    /// ASCII comma as a byte.
  291|       |    pub const COMMA_BYTE: u8 = Self::COMMA_CHAR as u8;
  292|       |
  293|       |    /// ASCII period.
  294|       |    pub const PERIOD_CHAR: char = '.';
  295|       |
  296|       |    /// ASCII period as a byte.
  297|       |    pub const PERIOD_BYTE: u8 = Self::PERIOD_CHAR as u8;
  298|       |
  299|       |    /// Minimum printable ASCII character.
  300|       |    pub const MIN_PRINTABLE_CHAR: char = ' ';
  301|       |
  302|       |    /// Minimum printable ASCII character as a byte.
  303|       |    pub const MIN_PRINTABLE_BYTE: u8 = Self::MIN_PRINTABLE_CHAR as u8;
  304|       |
  305|       |    /// Maximum printable ASCII character.
  306|       |    pub const MAX_PRINTABLE_CHAR: char = '~';
  307|       |
  308|       |    /// Maximum printable ASCII character as a byte.
  309|       |    pub const MAX_PRINTABLE_BYTE: u8 = Self::MAX_PRINTABLE_CHAR as u8;
  310|       |
  311|       |    /// All printable ASCII characters.
  312|       |    pub const PRINTABLE_CHARS: [char; 95] = [
  313|       |        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5',
  314|       |        '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
  315|       |        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a',
  316|       |        'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
  317|       |        'x', 'y', 'z', '{', '|', '}', '~',
  318|       |    ];
  319|       |
  320|       |    /// All printable ASCII characters as bytes.
  321|       |    pub const PRINTABLE_BYTES: [u8; 95] =
  322|       |        *b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
  323|       |
  324|       |    /// All ASCII letter characters.
  325|       |    pub const LETTER_CHARS: [char; 52] = [
  326|       |        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
  327|       |        'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
  328|       |        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  329|       |    ];
  330|       |
  331|       |    /// All ASCII letter characters as bytes.
  332|       |    pub const LETTER_BYTES: [u8; 52] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  333|       |
  334|       |    /// All ASCII letter and digit characters.
  335|       |    pub const LETTER_DIGIT_CHARS: [char; 62] = [
  336|       |        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  337|       |        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
  338|       |        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  339|       |    ];
  340|       |
  341|       |    /// All ASCII letter and digit characters as bytes.
  342|       |    pub const LETTER_DIGIT_BYTES: [u8; 62] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  343|       |
  344|       |    /// ASCII digit characters.
  345|       |    pub const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  346|       |
  347|       |    /// ASCII digit bytes.
  348|       |    pub const DIGIT_BYTES: [u8; 10] = *b"0123456789";
  349|       |
  350|       |    /// Lowercase ASCII hexadecimal digit characters.
  351|       |    pub const LOWERCASE_HEX_DIGIT_CHARS: [char; 16] = [
  352|       |        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
  353|       |    ];
  354|       |
  355|       |    /// Lowercase ASCII hexadecimal digit bytes.
  356|       |    pub const LOWERCASE_HEX_DIGIT_BYTES: [u8; 16] = *b"0123456789abcdef";
  357|       |
  358|       |    /// Uppercase ASCII hexadecimal digit characters.
  359|       |    pub const UPPERCASE_HEX_DIGIT_CHARS: [char; 16] = [
  360|       |        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
  361|       |    ];
  362|       |
  363|       |    /// Uppercase ASCII hexadecimal digit bytes.
  364|       |    pub const UPPERCASE_HEX_DIGIT_BYTES: [u8; 16] = *b"0123456789ABCDEF";
  365|       |
  366|       |    const CASE_DIFFERENCE: u32 = ('a' as u32) - ('A' as u32);
  367|       |
  368|       |    /// Tests whether a byte is an ASCII byte.
  369|       |    ///
  370|       |    /// # Parameters
  371|       |    ///
  372|       |    /// - `ch`: The byte to test.
  373|       |    ///
  374|       |    /// # Returns
  375|       |    ///
  376|       |    /// Returns `true` if `ch` is in the ASCII range `0x00..=0x7F`.
  377|       |    #[inline(always)]
  378|       |    #[must_use]
  379|      7|    pub const fn is_ascii_byte(ch: u8) -> bool {
  380|      7|        ch <= Self::MAX_BYTE
  381|      7|    }
  382|       |
  383|       |    /// Tests whether a character is an ASCII character.
  384|       |    ///
  385|       |    /// # Parameters
  386|       |    ///
  387|       |    /// - `ch`: The character to test.
  388|       |    ///
  389|       |    /// # Returns
  390|       |    ///
  391|       |    /// Returns `true` if `ch` is in the ASCII range `U+0000..=U+007F`.
  392|       |    #[inline]
  393|       |    #[must_use]
  394|     43|    pub const fn is_ascii_char(ch: char) -> bool {
  395|     43|        ch <= Self::MAX_CHAR
  396|     43|    }
  397|       |
  398|       |    /// Tests whether an integer value is an ASCII code point.
  399|       |    ///
  400|       |    /// # Parameters
  401|       |    ///
  402|       |    /// - `ch`: The raw code point value to test.
  403|       |    ///
  404|       |    /// # Returns
  405|       |    ///
  406|       |    /// Returns `true` if `ch` is in the ASCII range `0x00..=0x7F`.
  407|       |    /// Negative values and values above `0x7F` return `false`.
  408|       |    #[inline]
  409|       |    #[must_use]
  410|      3|    pub const fn is_ascii_code_point(ch: u32) -> bool {
  411|      3|        ch <= Self::MAX_CHAR as u32
  412|      3|    }
  413|       |
  414|       |    /// Tests whether a byte is Java-style ASCII whitespace.
  415|       |    ///
  416|       |    /// # Parameters
  417|       |    ///
  418|       |    /// - `ch`: The byte to test.
  419|       |    ///
  420|       |    /// # Returns
  421|       |    ///
  422|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
  423|       |    #[inline(always)]
  424|       |    #[must_use]
  425|      6|    pub const fn is_whitespace_byte(ch: u8) -> bool {
  426|      6|        ch == b'\t' || ch == b'\n' || ch == b'\x0c' || ch == b'\r' || ch == b' '
                                                    ^4               ^4             ^4
  427|      6|    }
  428|       |
  429|       |    /// Tests whether a character is Java-style ASCII whitespace.
  430|       |    ///
  431|       |    /// # Parameters
  432|       |    ///
  433|       |    /// - `ch`: The character to test.
  434|       |    ///
  435|       |    /// # Returns
  436|       |    ///
  437|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
  438|       |    #[inline]
  439|       |    #[must_use]
  440|      2|    pub const fn is_whitespace_char(ch: char) -> bool {
  441|      2|        ch == '\t' || ch == '\n' || ch == '\u{000c}' || ch == '\r' || ch == ' '
  442|      2|    }
  443|       |
  444|       |    /// Tests whether a raw code point is Java-style ASCII whitespace.
  445|       |    ///
  446|       |    /// # Parameters
  447|       |    ///
  448|       |    /// - `ch`: The raw code point value to test.
  449|       |    ///
  450|       |    /// # Returns
  451|       |    ///
  452|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
  453|       |    /// Other values, including negative values, return `false`.
  454|       |    #[inline]
  455|       |    #[must_use]
  456|      2|    pub const fn is_whitespace_code_point(ch: u32) -> bool {
  457|      2|        ch == '\t' as u32 || ch == '\n' as u32 || ch == '\u{000c}' as u32 || ch == '\r' as u32 || ch == ' ' as u32
                                                                                                                ^1
  458|      2|    }
  459|       |
  460|       |    /// Tests whether a byte is an ASCII letter.
  461|       |    ///
  462|       |    /// # Parameters
  463|       |    ///
  464|       |    /// - `ch`: The byte to test.
  465|       |    ///
  466|       |    /// # Returns
  467|       |    ///
  468|       |    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
  469|       |    #[inline(always)]
  470|       |    #[must_use]
  471|      3|    pub const fn is_letter_byte(ch: u8) -> bool {
  472|      3|        Self::is_uppercase_letter_byte(ch) || Self::is_lowercase_letter_byte(ch)
                                                            ^2                             ^2
  473|      3|    }
  474|       |
  475|       |    /// Tests whether a character is an ASCII letter.
  476|       |    ///
  477|       |    /// # Parameters
  478|       |    ///
  479|       |    /// - `ch`: The character to test.
  480|       |    ///
  481|       |    /// # Returns
  482|       |    ///
  483|       |    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
  484|       |    #[inline]
  485|       |    #[must_use]
  486|      3|    pub const fn is_letter_char(ch: char) -> bool {
  487|      3|        Self::is_uppercase_letter_char(ch) || Self::is_lowercase_letter_char(ch)
                                                            ^1                             ^1
  488|      3|    }
  489|       |
  490|       |    /// Tests whether a raw code point is an ASCII letter.
  491|       |    ///
  492|       |    /// # Parameters
  493|       |    ///
  494|       |    /// - `ch`: The raw code point value to test.
  495|       |    ///
  496|       |    /// # Returns
  497|       |    ///
  498|       |    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
  499|       |    #[inline]
  500|       |    #[must_use]
  501|      3|    pub const fn is_letter_code_point(ch: u32) -> bool {
  502|      3|        Self::is_uppercase_letter_code_point(ch) || Self::is_lowercase_letter_code_point(ch)
                                                                  ^2                                   ^2
  503|      3|    }
  504|       |
  505|       |    /// Tests whether a byte is an uppercase ASCII letter.
  506|       |    ///
  507|       |    /// # Parameters
  508|       |    ///
  509|       |    /// - `ch`: The byte to test.
  510|       |    ///
  511|       |    /// # Returns
  512|       |    ///
  513|       |    /// Returns `true` if `ch` is in `A..=Z`.
  514|       |    #[inline(always)]
  515|       |    #[must_use]
  516|     30|    pub const fn is_uppercase_letter_byte(ch: u8) -> bool {
  517|     30|        ch >= b'A' && ch <= b'Z'
                                    ^28
  518|     30|    }
  519|       |
  520|       |    /// Tests whether a character is an uppercase ASCII letter.
  521|       |    ///
  522|       |    /// # Parameters
  523|       |    ///
  524|       |    /// - `ch`: The character to test.
  525|       |    ///
  526|       |    /// # Returns
  527|       |    ///
  528|       |    /// Returns `true` if `ch` is in `A..=Z`.
  529|       |    #[inline]
  530|       |    #[must_use]
  531|      4|    pub const fn is_uppercase_letter_char(ch: char) -> bool {
  532|      4|        ch >= 'A' && ch <= 'Z'
  533|      4|    }
  534|       |
  535|       |    /// Tests whether a raw code point is an uppercase ASCII letter.
  536|       |    ///
  537|       |    /// # Parameters
  538|       |    ///
  539|       |    /// - `ch`: The raw code point value to test.
  540|       |    ///
  541|       |    /// # Returns
  542|       |    ///
  543|       |    /// Returns `true` if `ch` is in `A..=Z`.
  544|       |    #[inline]
  545|       |    #[must_use]
  546|      4|    pub const fn is_uppercase_letter_code_point(ch: u32) -> bool {
  547|      4|        ch >= 'A' as u32 && ch <= 'Z' as u32
  548|      4|    }
  549|       |
  550|       |    /// Tests whether a byte is a lowercase ASCII letter.
  551|       |    ///
  552|       |    /// # Parameters
  553|       |    ///
  554|       |    /// - `ch`: The byte to test.
  555|       |    ///
  556|       |    /// # Returns
  557|       |    ///
  558|       |    /// Returns `true` if `ch` is in `a..=z`.
  559|       |    #[inline(always)]
  560|       |    #[must_use]
  561|     22|    pub const fn is_lowercase_letter_byte(ch: u8) -> bool {
  562|     22|        ch >= b'a' && ch <= b'z'
                                    ^20
  563|     22|    }
  564|       |
  565|       |    /// Tests whether a character is a lowercase ASCII letter.
  566|       |    ///
  567|       |    /// # Parameters
  568|       |    ///
  569|       |    /// - `ch`: The character to test.
  570|       |    ///
  571|       |    /// # Returns
  572|       |    ///
  573|       |    /// Returns `true` if `ch` is in `a..=z`.
  574|       |    #[inline]
  575|       |    #[must_use]
  576|      2|    pub const fn is_lowercase_letter_char(ch: char) -> bool {
  577|      2|        ch >= 'a' && ch <= 'z'
  578|      2|    }
  579|       |
  580|       |    /// Tests whether a raw code point is a lowercase ASCII letter.
  581|       |    ///
  582|       |    /// # Parameters
  583|       |    ///
  584|       |    /// - `ch`: The raw code point value to test.
  585|       |    ///
  586|       |    /// # Returns
  587|       |    ///
  588|       |    /// Returns `true` if `ch` is in `a..=z`.
  589|       |    #[inline]
  590|       |    #[must_use]
  591|      3|    pub const fn is_lowercase_letter_code_point(ch: u32) -> bool {
  592|      3|        ch >= 'a' as u32 && ch <= 'z' as u32
  593|      3|    }
  594|       |
  595|       |    /// Tests whether a byte is an ASCII decimal digit.
  596|       |    ///
  597|       |    /// # Parameters
  598|       |    ///
  599|       |    /// - `ch`: The byte to test.
  600|       |    ///
  601|       |    /// # Returns
  602|       |    ///
  603|       |    /// Returns `true` if `ch` is in `0..=9`.
  604|       |    #[inline(always)]
  605|       |    #[must_use]
  606|     15|    pub const fn is_digit_byte(ch: u8) -> bool {
  607|     15|        ch >= b'0' && ch <= b'9'
  608|     15|    }
  609|       |
  610|       |    /// Tests whether a character is an ASCII decimal digit.
  611|       |    ///
  612|       |    /// # Parameters
  613|       |    ///
  614|       |    /// - `ch`: The character to test.
  615|       |    ///
  616|       |    /// # Returns
  617|       |    ///
  618|       |    /// Returns `true` if `ch` is in `0..=9`.
  619|       |    #[inline]
  620|       |    #[must_use]
  621|      4|    pub const fn is_digit_char(ch: char) -> bool {
  622|      4|        ch >= '0' && ch <= '9'
  623|      4|    }
  624|       |
  625|       |    /// Tests whether a raw code point is an ASCII decimal digit.
  626|       |    ///
  627|       |    /// # Parameters
  628|       |    ///
  629|       |    /// - `ch`: The raw code point value to test.
  630|       |    ///
  631|       |    /// # Returns
  632|       |    ///
  633|       |    /// Returns `true` if `ch` is in `0..=9`.
  634|       |    #[inline]
  635|       |    #[must_use]
  636|      6|    pub const fn is_digit_code_point(ch: u32) -> bool {
  637|      6|        ch >= '0' as u32 && ch <= '9' as u32
  638|      6|    }
  639|       |
  640|       |    /// Tests whether a byte is an ASCII hexadecimal digit.
  641|       |    ///
  642|       |    /// # Parameters
  643|       |    ///
  644|       |    /// - `ch`: The byte to test.
  645|       |    ///
  646|       |    /// # Returns
  647|       |    ///
  648|       |    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
  649|       |    #[inline(always)]
  650|       |    #[must_use]
  651|      3|    pub const fn is_hex_digit_byte(ch: u8) -> bool {
  652|      3|        Self::is_digit_byte(ch) || (ch >= b'a' && ch <= b'f') || (ch >= b'A' && ch <= b'F')
                                                                ^2              ^1            ^1
  653|      3|    }
  654|       |
  655|       |    /// Tests whether a character is an ASCII hexadecimal digit.
  656|       |    ///
  657|       |    /// # Parameters
  658|       |    ///
  659|       |    /// - `ch`: The character to test.
  660|       |    ///
  661|       |    /// # Returns
  662|       |    ///
  663|       |    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
  664|       |    #[inline]
  665|       |    #[must_use]
  666|      1|    pub const fn is_hex_digit_char(ch: char) -> bool {
  667|      1|        Self::is_digit_char(ch) || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
                                                               ^0
  668|      1|    }
  669|       |
  670|       |    /// Tests whether a raw code point is an ASCII hexadecimal digit.
  671|       |    ///
  672|       |    /// # Parameters
  673|       |    ///
  674|       |    /// - `ch`: The raw code point value to test.
  675|       |    ///
  676|       |    /// # Returns
  677|       |    ///
  678|       |    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
  679|       |    #[inline]
  680|       |    #[must_use]
  681|      3|    pub const fn is_hex_digit_code_point(ch: u32) -> bool {
  682|      3|        Self::is_digit_code_point(ch)
  683|      2|            || (ch >= 'a' as u32 && ch <= 'f' as u32)
                                                  ^1
  684|      1|            || (ch >= 'A' as u32 && ch <= 'F' as u32)
  685|      3|    }
  686|       |
  687|       |    /// Tests whether a byte is an ASCII octal digit.
  688|       |    ///
  689|       |    /// # Parameters
  690|       |    ///
  691|       |    /// - `ch`: The byte to test.
  692|       |    ///
  693|       |    /// # Returns
  694|       |    ///
  695|       |    /// Returns `true` if `ch` is in `0..=7`.
  696|       |    #[inline(always)]
  697|       |    #[must_use]
  698|      4|    pub const fn is_octal_digit_byte(ch: u8) -> bool {
  699|      4|        ch >= b'0' && ch <= b'7'
  700|      4|    }
  701|       |
  702|       |    /// Tests whether a character is an ASCII octal digit.
  703|       |    ///
  704|       |    /// # Parameters
  705|       |    ///
  706|       |    /// - `ch`: The character to test.
  707|       |    ///
  708|       |    /// # Returns
  709|       |    ///
  710|       |    /// Returns `true` if `ch` is in `0..=7`.
  711|       |    #[inline]
  712|       |    #[must_use]
  713|      2|    pub const fn is_octal_digit_char(ch: char) -> bool {
  714|      2|        ch >= '0' && ch <= '7'
  715|      2|    }
  716|       |
  717|       |    /// Tests whether a raw code point is an ASCII octal digit.
  718|       |    ///
  719|       |    /// # Parameters
  720|       |    ///
  721|       |    /// - `ch`: The raw code point value to test.
  722|       |    ///
  723|       |    /// # Returns
  724|       |    ///
  725|       |    /// Returns `true` if `ch` is in `0..=7`.
  726|       |    #[inline]
  727|       |    #[must_use]
  728|      2|    pub const fn is_octal_digit_code_point(ch: u32) -> bool {
  729|      2|        ch >= '0' as u32 && ch <= '7' as u32
  730|      2|    }
  731|       |
  732|       |    /// Tests whether a byte is an ASCII letter or digit.
  733|       |    ///
  734|       |    /// # Parameters
  735|       |    ///
  736|       |    /// - `ch`: The byte to test.
  737|       |    ///
  738|       |    /// # Returns
  739|       |    ///
  740|       |    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
  741|       |    #[inline(always)]
  742|       |    #[must_use]
  743|      1|    pub const fn is_letter_or_digit_byte(ch: u8) -> bool {
  744|      1|        Self::is_letter_byte(ch) || Self::is_digit_byte(ch)
  745|      1|    }
  746|       |
  747|       |    /// Tests whether a character is an ASCII letter or digit.
  748|       |    ///
  749|       |    /// # Parameters
  750|       |    ///
  751|       |    /// - `ch`: The character to test.
  752|       |    ///
  753|       |    /// # Returns
  754|       |    ///
  755|       |    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
  756|       |    #[inline]
  757|       |    #[must_use]
  758|      1|    pub const fn is_letter_or_digit_char(ch: char) -> bool {
  759|      1|        Self::is_letter_char(ch) || Self::is_digit_char(ch)
                                                  ^0                  ^0
  760|      1|    }
  761|       |
  762|       |    /// Tests whether a raw code point is an ASCII letter or digit.
  763|       |    ///
  764|       |    /// # Parameters
  765|       |    ///
  766|       |    /// - `ch`: The raw code point value to test.
  767|       |    ///
  768|       |    /// # Returns
  769|       |    ///
  770|       |    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
  771|       |    #[inline]
  772|       |    #[must_use]
  773|      1|    pub const fn is_letter_or_digit_code_point(ch: u32) -> bool {
  774|      1|        Self::is_letter_code_point(ch) || Self::is_digit_code_point(ch)
                                                        ^0                        ^0
  775|      1|    }
  776|       |
  777|       |    /// Tests whether a byte is a printable ASCII character.
  778|       |    ///
  779|       |    /// # Parameters
  780|       |    ///
  781|       |    /// - `ch`: The byte to test.
  782|       |    ///
  783|       |    /// # Returns
  784|       |    ///
  785|       |    /// Returns `true` if `ch` is in the printable ASCII range `0x20..=0x7E`.
  786|       |    #[inline(always)]
  787|       |    #[must_use]
  788|      1|    pub const fn is_printable_byte(ch: u8) -> bool {
  789|      1|        ch >= Self::MIN_PRINTABLE_BYTE && ch <= Self::MAX_PRINTABLE_BYTE
  790|      1|    }
  791|       |
  792|       |    /// Tests whether a character is a printable ASCII character.
  793|       |    ///
  794|       |    /// # Parameters
  795|       |    ///
  796|       |    /// - `ch`: The character to test.
  797|       |    ///
  798|       |    /// # Returns
  799|       |    ///
  800|       |    /// Returns `true` if `ch` is in the printable ASCII range
  801|       |    /// `U+0020..=U+007E`.
  802|       |    #[inline]
  803|       |    #[must_use]
  804|      1|    pub const fn is_printable_char(ch: char) -> bool {
  805|      1|        ch >= Self::MIN_PRINTABLE_CHAR && ch <= Self::MAX_PRINTABLE_CHAR
  806|      1|    }
  807|       |
  808|       |    /// Tests whether a raw code point is a printable ASCII character.
  809|       |    ///
  810|       |    /// # Parameters
  811|       |    ///
  812|       |    /// - `ch`: The raw code point value to test.
  813|       |    ///
  814|       |    /// # Returns
  815|       |    ///
  816|       |    /// Returns `true` if `ch` is in the printable ASCII range `0x20..=0x7E`.
  817|       |    #[inline]
  818|       |    #[must_use]
  819|      1|    pub const fn is_printable_code_point(ch: u32) -> bool {
  820|      1|        ch >= Self::MIN_PRINTABLE_CHAR as u32 && ch <= Self::MAX_PRINTABLE_CHAR as u32
  821|      1|    }
  822|       |
  823|       |    /// Tests whether a byte is an ASCII control character.
  824|       |    ///
  825|       |    /// # Parameters
  826|       |    ///
  827|       |    /// - `ch`: The byte to test.
  828|       |    ///
  829|       |    /// # Returns
  830|       |    ///
  831|       |    /// Returns `true` for `0x00..=0x1F` or `0x7F`.
  832|       |    #[inline(always)]
  833|       |    #[must_use]
  834|      1|    pub const fn is_control_byte(ch: u8) -> bool {
  835|      1|        ch < Self::MIN_PRINTABLE_BYTE || ch == Self::DELETE_BYTE
                                                       ^0
  836|      1|    }
  837|       |
  838|       |    /// Tests whether a character is an ASCII control character.
  839|       |    ///
  840|       |    /// # Parameters
  841|       |    ///
  842|       |    /// - `ch`: The character to test.
  843|       |    ///
  844|       |    /// # Returns
  845|       |    ///
  846|       |    /// Returns `true` for `U+0000..=U+001F` or `U+007F`.
  847|       |    #[inline]
  848|       |    #[must_use]
  849|      1|    pub const fn is_control_char(ch: char) -> bool {
  850|      1|        (ch < Self::MIN_PRINTABLE_CHAR) || ch == Self::DELETE_CHAR
                                                         ^0
  851|      1|    }
  852|       |
  853|       |    /// Tests whether a raw code point is an ASCII control character.
  854|       |    ///
  855|       |    /// # Parameters
  856|       |    ///
  857|       |    /// - `ch`: The raw code point value to test.
  858|       |    ///
  859|       |    /// # Returns
  860|       |    ///
  861|       |    /// Returns `true` for `0x00..=0x1F` or `0x7F`.
  862|       |    #[inline]
  863|       |    #[must_use]
  864|      1|    pub const fn is_control_code_point(ch: u32) -> bool {
  865|      1|        (ch < Self::MIN_PRINTABLE_CHAR as u32) || ch == Self::DELETE_CHAR as u32
                                                                ^0
  866|      1|    }
  867|       |
  868|       |    /// Compares two bytes while ignoring ASCII case.
  869|       |    ///
  870|       |    /// # Parameters
  871|       |    ///
  872|       |    /// - `ch1`: The first byte to compare.
  873|       |    /// - `ch2`: The second byte to compare.
  874|       |    ///
  875|       |    /// # Returns
  876|       |    ///
  877|       |    /// Returns `true` if the bytes are equal after converting ASCII uppercase
  878|       |    /// letters to lowercase. Non-ASCII bytes are compared unchanged.
  879|       |    #[inline(always)]
  880|       |    #[must_use]
  881|      3|    pub const fn equals_ignore_case_byte(ch1: u8, ch2: u8) -> bool {
  882|      3|        if ch1 == ch2 {
  883|      1|            true
  884|       |        } else {
  885|      2|            Self::byte_to_lowercase(ch1) == Self::byte_to_lowercase(ch2)
  886|       |        }
  887|      3|    }
  888|       |
  889|       |    /// Compares two characters while ignoring ASCII case.
  890|       |    ///
  891|       |    /// # Parameters
  892|       |    ///
  893|       |    /// - `ch1`: The first character to compare.
  894|       |    /// - `ch2`: The second character to compare.
  895|       |    ///
  896|       |    /// # Returns
  897|       |    ///
  898|       |    /// Returns `true` if the characters are equal after converting ASCII
  899|       |    /// uppercase letters to lowercase. Non-ASCII characters are compared
  900|       |    /// unchanged.
  901|       |    #[inline]
  902|       |    #[must_use]
  903|      2|    pub const fn equals_ignore_case_char(ch1: char, ch2: char) -> bool {
  904|      2|        if ch1 == ch2 {
  905|      1|            true
  906|       |        } else {
  907|      1|            Self::char_to_lowercase(ch1) == Self::char_to_lowercase(ch2)
  908|       |        }
  909|      2|    }
  910|       |
  911|       |    /// Compares two raw code points while ignoring ASCII case.
  912|       |    ///
  913|       |    /// # Parameters
  914|       |    ///
  915|       |    /// - `ch1`: The first raw code point value to compare.
  916|       |    /// - `ch2`: The second raw code point value to compare.
  917|       |    ///
  918|       |    /// # Returns
  919|       |    ///
  920|       |    /// Returns `true` if the values are equal after converting ASCII uppercase
  921|       |    /// letters to lowercase. Values outside ASCII are compared unchanged.
  922|       |    #[inline]
  923|       |    #[must_use]
  924|      2|    pub const fn equals_ignore_case_code_point(ch1: u32, ch2: u32) -> bool {
  925|      2|        if ch1 == ch2 {
  926|      1|            true
  927|       |        } else {
  928|      1|            Self::code_point_to_lowercase(ch1) == Self::code_point_to_lowercase(ch2)
  929|       |        }
  930|      2|    }
  931|       |
  932|       |    /// Converts a byte to uppercase using ASCII case rules.
  933|       |    ///
  934|       |    /// # Parameters
  935|       |    ///
  936|       |    /// - `ch`: The byte to convert.
  937|       |    ///
  938|       |    /// # Returns
  939|       |    ///
  940|       |    /// Returns the uppercase ASCII equivalent for `a..=z`; all other bytes are
  941|       |    /// returned unchanged.
  942|       |    #[inline(always)]
  943|       |    #[must_use]
  944|      2|    pub const fn byte_to_uppercase(ch: u8) -> u8 {
  945|      2|        if ch >= b'a' && ch <= b'z' {
                                       ^1
  946|      1|            ch - (Self::CASE_DIFFERENCE as u8)
  947|       |        } else {
  948|      1|            ch
  949|       |        }
  950|      2|    }
  951|       |
  952|       |    /// Converts a character to uppercase using ASCII case rules.
  953|       |    ///
  954|       |    /// # Parameters
  955|       |    ///
  956|       |    /// - `ch`: The character to convert.
  957|       |    ///
  958|       |    /// # Returns
  959|       |    ///
  960|       |    /// Returns the uppercase ASCII equivalent for `a..=z`; all other characters
  961|       |    /// are returned unchanged.
  962|       |    #[inline]
  963|       |    #[must_use]
  964|      2|    pub const fn char_to_uppercase(ch: char) -> char {
  965|      2|        if ch >= 'a' && ch <= 'z' {
                                      ^1
  966|      1|            ((ch as u8) - (Self::CASE_DIFFERENCE as u8)) as char
  967|       |        } else {
  968|      1|            ch
  969|       |        }
  970|      2|    }
  971|       |
  972|       |    /// Converts a raw code point to uppercase using ASCII case rules.
  973|       |    ///
  974|       |    /// # Parameters
  975|       |    ///
  976|       |    /// - `ch`: The raw code point value to convert.
  977|       |    ///
  978|       |    /// # Returns
  979|       |    ///
  980|       |    /// Returns the uppercase ASCII equivalent for `a..=z`; all other values are
  981|       |    /// returned unchanged.
  982|       |    #[inline]
  983|       |    #[must_use]
  984|      2|    pub const fn code_point_to_uppercase(ch: u32) -> u32 {
  985|      2|        if ch >= 'a' as u32 && ch <= 'z' as u32 {
                                             ^1
  986|      1|            ch - Self::CASE_DIFFERENCE
  987|       |        } else {
  988|      1|            ch
  989|       |        }
  990|      2|    }
  991|       |
  992|       |    /// Converts a byte to lowercase using ASCII case rules.
  993|       |    ///
  994|       |    /// # Parameters
  995|       |    ///
  996|       |    /// - `ch`: The byte to convert.
  997|       |    ///
  998|       |    /// # Returns
  999|       |    ///
 1000|       |    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other bytes are
 1001|       |    /// returned unchanged.
 1002|       |    #[inline(always)]
 1003|       |    #[must_use]
 1004|      6|    pub const fn byte_to_lowercase(ch: u8) -> u8 {
 1005|      6|        if ch >= b'A' && ch <= b'Z' {
 1006|      4|            ch + (Self::CASE_DIFFERENCE as u8)
 1007|       |        } else {
 1008|      2|            ch
 1009|       |        }
 1010|      6|    }
 1011|       |
 1012|       |    /// Converts a character to lowercase using ASCII case rules.
 1013|       |    ///
 1014|       |    /// # Parameters
 1015|       |    ///
 1016|       |    /// - `ch`: The character to convert.
 1017|       |    ///
 1018|       |    /// # Returns
 1019|       |    ///
 1020|       |    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other characters
 1021|       |    /// are returned unchanged.
 1022|       |    #[inline]
 1023|       |    #[must_use]
 1024|      4|    pub const fn char_to_lowercase(ch: char) -> char {
 1025|      4|        if ch >= 'A' && ch <= 'Z' {
 1026|      2|            ((ch as u8) + (Self::CASE_DIFFERENCE as u8)) as char
 1027|       |        } else {
 1028|      2|            ch
 1029|       |        }
 1030|      4|    }
 1031|       |
 1032|       |    /// Converts a raw code point to lowercase using ASCII case rules.
 1033|       |    ///
 1034|       |    /// # Parameters
 1035|       |    ///
 1036|       |    /// - `ch`: The raw code point value to convert.
 1037|       |    ///
 1038|       |    /// # Returns
 1039|       |    ///
 1040|       |    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other values are
 1041|       |    /// returned unchanged.
 1042|       |    #[inline]
 1043|       |    #[must_use]
 1044|      3|    pub const fn code_point_to_lowercase(ch: u32) -> u32 {
 1045|      3|        if ch >= 'A' as u32 && ch <= 'Z' as u32 {
 1046|      2|            ch + Self::CASE_DIFFERENCE
 1047|       |        } else {
 1048|      1|            ch
 1049|       |        }
 1050|      3|    }
 1051|       |
 1052|       |    /// Converts an ASCII decimal digit byte into its numeric value.
 1053|       |    ///
 1054|       |    /// # Parameters
 1055|       |    ///
 1056|       |    /// - `ch`: The byte to convert.
 1057|       |    ///
 1058|       |    /// # Returns
 1059|       |    ///
 1060|       |    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
 1061|       |    #[inline(always)]
 1062|       |    #[must_use]
 1063|      6|    pub const fn byte_to_digit(ch: u8) -> Option<u8> {
 1064|      6|        if Self::is_digit_byte(ch) { Some(ch - b'0') } else { None }
                                                   ^4                       ^2
 1065|      6|    }
 1066|       |
 1067|       |    /// Converts an ASCII decimal digit character into its numeric value.
 1068|       |    ///
 1069|       |    /// # Parameters
 1070|       |    ///
 1071|       |    /// - `ch`: The character to convert.
 1072|       |    ///
 1073|       |    /// # Returns
 1074|       |    ///
 1075|       |    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
 1076|       |    #[inline]
 1077|       |    #[must_use]
 1078|      2|    pub const fn char_to_digit(ch: char) -> Option<u8> {
 1079|      2|        if Self::is_digit_char(ch) {
 1080|      1|            Some((ch as u8) - b'0')
 1081|       |        } else {
 1082|      1|            None
 1083|       |        }
 1084|      2|    }
 1085|       |
 1086|       |    /// Converts an ASCII decimal digit code point into its numeric value.
 1087|       |    ///
 1088|       |    /// # Parameters
 1089|       |    ///
 1090|       |    /// - `ch`: The raw code point value to convert.
 1091|       |    ///
 1092|       |    /// # Returns
 1093|       |    ///
 1094|       |    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
 1095|       |    #[inline]
 1096|       |    #[must_use]
 1097|      2|    pub const fn code_point_to_digit(ch: u32) -> Option<u8> {
 1098|      2|        if Self::is_digit_code_point(ch) {
 1099|      1|            Some((ch - '0' as u32) as u8)
 1100|       |        } else {
 1101|      1|            None
 1102|       |        }
 1103|      2|    }
 1104|       |
 1105|       |    /// Converts an ASCII hexadecimal digit byte into its numeric value.
 1106|       |    ///
 1107|       |    /// # Parameters
 1108|       |    ///
 1109|       |    /// - `ch`: The byte to convert.
 1110|       |    ///
 1111|       |    /// # Returns
 1112|       |    ///
 1113|       |    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
 1114|       |    /// otherwise.
 1115|       |    #[inline(always)]
 1116|       |    #[must_use]
 1117|      7|    pub const fn byte_to_hex_digit(ch: u8) -> Option<u8> {
 1118|      7|        if ch >= b'0' && ch <= b'9' {
 1119|      1|            Some(ch - b'0')
 1120|      6|        } else if ch >= b'A' && ch <= b'F' {
 1121|      2|            Some(ch - (b'A' - 10))
 1122|      4|        } else if ch >= b'a' && ch <= b'f' {
 1123|      2|            Some(ch - (b'a' - 10))
 1124|       |        } else {
 1125|      2|            None
 1126|       |        }
 1127|      7|    }
 1128|       |
 1129|       |    /// Converts an ASCII hexadecimal digit character into its numeric value.
 1130|       |    ///
 1131|       |    /// # Parameters
 1132|       |    ///
 1133|       |    /// - `ch`: The character to convert.
 1134|       |    ///
 1135|       |    /// # Returns
 1136|       |    ///
 1137|       |    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
 1138|       |    /// otherwise.
 1139|       |    #[inline]
 1140|       |    #[must_use]
 1141|      4|    pub const fn char_to_hex_digit(ch: char) -> Option<u8> {
 1142|      4|        if ch >= '0' && ch <= '9' {
 1143|      1|            Some((ch as u8) - b'0')
 1144|      3|        } else if ch >= 'A' && ch <= 'F' {
 1145|      1|            Some((ch as u8) - (b'A' - 10))
 1146|      2|        } else if ch >= 'a' && ch <= 'f' {
 1147|      1|            Some((ch as u8) - (b'a' - 10))
 1148|       |        } else {
 1149|      1|            None
 1150|       |        }
 1151|      4|    }
 1152|       |
 1153|       |    /// Converts an ASCII hexadecimal digit code point into its numeric value.
 1154|       |    ///
 1155|       |    /// # Parameters
 1156|       |    ///
 1157|       |    /// - `ch`: The raw code point value to convert.
 1158|       |    ///
 1159|       |    /// # Returns
 1160|       |    ///
 1161|       |    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
 1162|       |    /// otherwise.
 1163|       |    #[inline]
 1164|       |    #[must_use]
 1165|      4|    pub const fn code_point_to_hex_digit(ch: u32) -> Option<u8> {
 1166|      4|        if ch >= '0' as u32 && ch <= '9' as u32 {
 1167|      1|            Some((ch - '0' as u32) as u8)
 1168|      3|        } else if ch >= 'A' as u32 && ch <= 'F' as u32 {
 1169|      1|            Some((ch - ('A' as u32 - 10)) as u8)
 1170|      2|        } else if ch >= 'a' as u32 && ch <= 'f' as u32 {
 1171|      1|            Some((ch - ('a' as u32 - 10)) as u8)
 1172|       |        } else {
 1173|      1|            None
 1174|       |        }
 1175|      4|    }
 1176|       |
 1177|       |    /// Folds a Unicode character to its ASCII replacement.
 1178|       |    ///
 1179|       |    /// # Parameters
 1180|       |    ///
 1181|       |    /// - `ch`: The character to fold.
 1182|       |    /// - `result`: The caller-provided output buffer that receives the folded
 1183|       |    ///   characters.
 1184|       |    /// - `offset`: The starting index in `result` at which folded characters
 1185|       |    ///   are written.
 1186|       |    ///
 1187|       |    /// # Returns
 1188|       |    ///
 1189|       |    /// Returns the number of characters written to `result` starting at
 1190|       |    /// `offset`. ASCII characters and unmapped non-ASCII characters write one
 1191|       |    /// character. Mapped characters write up to [`Self::MAX_FOLDING_COUNT`]
 1192|       |    /// ASCII characters.
 1193|       |    ///
 1194|       |    /// # Panics
 1195|       |    ///
 1196|       |    /// Panics if `result` has fewer than [`Self::MAX_FOLDING_COUNT`] writable
 1197|       |    /// slots after `offset`.
 1198|  1.24k|    pub fn fold(ch: char, result: &mut [char], offset: usize) -> usize {
 1199|  1.24k|        assert!(
 1200|  1.24k|            result.len().saturating_sub(offset) >= Self::MAX_FOLDING_COUNT,
 1201|       |            "ASCII folding output needs at least MAX_FOLDING_COUNT slots"
 1202|       |        );
 1203|  1.24k|        if ch.is_ascii() {
 1204|      1|            result[offset] = ch;
 1205|      1|            return 1;
 1206|  1.24k|        }
 1207|  1.24k|        match ascii_folding::fold_replacement(ch) {
 1208|  1.24k|            Some(replacement) => {
 1209|  1.49k|                for (index, replacement_char) in replacement.chars().enumerate() {
                                                               ^1.24k      ^1.24k  ^1.24k
 1210|  1.49k|                    result[offset + index] = replacement_char;
 1211|  1.49k|                }
 1212|  1.24k|                replacement.len()
 1213|       |            }
 1214|       |            None => {
 1215|      1|                result[offset] = ch;
 1216|      1|                1
 1217|       |            }
 1218|       |        }
 1219|  1.24k|    }
 1220|       |
 1221|       |    /// Folds a Unicode character into an owned string.
 1222|       |    ///
 1223|       |    /// # Parameters
 1224|       |    ///
 1225|       |    /// - `ch`: The character to fold.
 1226|       |    ///
 1227|       |    /// # Returns
 1228|       |    ///
 1229|       |    /// Returns the folded ASCII replacement as a `String`. If `ch` has no
 1230|       |    /// folding mapping, the returned string contains `ch` unchanged.
 1231|       |    #[must_use]
 1232|      1|    pub fn fold_to_string(ch: char) -> String {
 1233|      1|        let mut buffer = ['\0'; Self::MAX_FOLDING_COUNT];
 1234|      1|        let count = Self::fold(ch, &mut buffer, 0);
 1235|      1|        buffer[..count].iter().collect()
 1236|      1|    }
 1237|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/ascii_folding.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |//! ASCII folding table ported from Qubit Java common utilities.
    9|       |
   10|       |/// Returns the ASCII folding replacement for a non-ASCII character.
   11|       |///
   12|       |/// This table is ported from Java `ltd.qubit.commons.text.Ascii#fold`,
   13|       |/// including the same Unicode code points and replacement values. The Java
   14|       |/// source keeps per-entry inline annotations such as `// À  [LATIN CAPITAL
   15|       |/// LETTER A WITH GRAVE]` to show the original glyph and Unicode name; this Rust
   16|       |/// port keeps those annotations beside the escaped code points while preserving
   17|       |/// identical mapping behavior.
   18|       |///
   19|       |/// # Parameters
   20|       |///
   21|       |/// - `ch`: The non-ASCII character to look up.
   22|       |///
   23|       |/// # Returns
   24|       |///
   25|       |/// Returns `Some(replacement)` when `ch` has a known ASCII folding replacement.
   26|       |/// Returns `None` when `ch` has no mapping in the table.
   27|       |#[must_use]
   28|  1.24k|pub(crate) fn fold_replacement(ch: char) -> Option<&'static str> {
   29|  1.24k|    match ch {
   30|      1|        '\u{00C0}' => Some("A"), // À  [LATIN CAPITAL LETTER A WITH GRAVE]
   31|      1|        '\u{00C1}' => Some("A"), // Á  [LATIN CAPITAL LETTER A WITH ACUTE]
   32|      1|        '\u{00C2}' => Some("A"), // Â  [LATIN CAPITAL LETTER A WITH CIRCUMFLEX]
   33|      1|        '\u{00C3}' => Some("A"), // Ã  [LATIN CAPITAL LETTER A WITH TILDE]
   34|      1|        '\u{00C4}' => Some("A"), // Ä  [LATIN CAPITAL LETTER A WITH DIAERESIS]
   35|      1|        '\u{00C5}' => Some("A"), // Å  [LATIN CAPITAL LETTER A WITH RING ABOVE]
   36|      1|        '\u{0100}' => Some("A"), // Ā  [LATIN CAPITAL LETTER A WITH MACRON]
   37|      1|        '\u{0102}' => Some("A"), // Ă  [LATIN CAPITAL LETTER A WITH BREVE]
   38|      1|        '\u{0104}' => Some("A"), // Ą  [LATIN CAPITAL LETTER A WITH OGONEK]
   39|      1|        '\u{018F}' => Some("A"), /* Ə  http://en.wikipedia.org/wiki/Schwa  [LATIN CAPITAL LETTER SCHWA] */
   40|      1|        '\u{01CD}' => Some("A"), // Ǎ  [LATIN CAPITAL LETTER A WITH CARON]
   41|      1|        '\u{01DE}' => Some("A"), /* Ǟ  [LATIN CAPITAL LETTER A WITH */
   42|       |        // DIAERESIS AND MACRON]
   43|      1|        '\u{01E0}' => Some("A"), /* Ǡ  [LATIN CAPITAL LETTER A WITH DOT */
   44|       |        // ABOVE AND MACRON]
   45|      1|        '\u{01FA}' => Some("A"), /* Ǻ  [LATIN CAPITAL LETTER A WITH RING */
   46|       |        // ABOVE AND ACUTE]
   47|      1|        '\u{0200}' => Some("A"), /* Ȁ  [LATIN CAPITAL LETTER A WITH DOUBLE */
   48|       |        // GRAVE]
   49|      1|        '\u{0202}' => Some("A"), /* Ȃ  [LATIN CAPITAL LETTER A WITH INVERTED */
   50|       |        // BREVE]
   51|      1|        '\u{0226}' => Some("A"), // Ȧ  [LATIN CAPITAL LETTER A WITH DOT ABOVE]
   52|      1|        '\u{023A}' => Some("A"), // Ⱥ  [LATIN CAPITAL LETTER A WITH STROKE]
   53|      1|        '\u{1D00}' => Some("A"), // ᴀ  [LATIN LETTER SMALL CAPITAL A]
   54|      1|        '\u{1E00}' => Some("A"), // Ḁ  [LATIN CAPITAL LETTER A WITH RING BELOW]
   55|      1|        '\u{1EA0}' => Some("A"), // Ạ  [LATIN CAPITAL LETTER A WITH DOT BELOW]
   56|      1|        '\u{1EA2}' => Some("A"), // Ả  [LATIN CAPITAL LETTER A WITH HOOK ABOVE]
   57|      1|        '\u{1EA4}' => Some("A"), /* Ấ  [LATIN CAPITAL LETTER A WITH */
   58|       |        // CIRCUMFLEX AND ACUTE]
   59|      1|        '\u{1EA6}' => Some("A"), /* Ầ  [LATIN CAPITAL LETTER A WITH */
   60|       |        // CIRCUMFLEX AND GRAVE]
   61|      1|        '\u{1EA8}' => Some("A"), /* Ẩ  [LATIN CAPITAL LETTER A WITH */
   62|       |        // CIRCUMFLEX AND HOOK ABOVE]
   63|      1|        '\u{1EAA}' => Some("A"), /* Ẫ  [LATIN CAPITAL LETTER A WITH */
   64|       |        // CIRCUMFLEX AND TILDE]
   65|      1|        '\u{1EAC}' => Some("A"), /* Ậ  [LATIN CAPITAL LETTER A WITH */
   66|       |        // CIRCUMFLEX AND DOT BELOW]
   67|      1|        '\u{1EAE}' => Some("A"), /* Ắ  [LATIN CAPITAL LETTER A WITH BREVE */
   68|       |        // AND ACUTE]
   69|      1|        '\u{1EB0}' => Some("A"), /* Ằ  [LATIN CAPITAL LETTER A WITH BREVE */
   70|       |        // AND GRAVE]
   71|      1|        '\u{1EB2}' => Some("A"), /* Ẳ  [LATIN CAPITAL LETTER A WITH BREVE */
   72|       |        // AND HOOK ABOVE]
   73|      1|        '\u{1EB4}' => Some("A"), /* Ẵ  [LATIN CAPITAL LETTER A WITH BREVE */
   74|       |        // AND TILDE]
   75|      1|        '\u{1EB6}' => Some("A"), /* Ặ  [LATIN CAPITAL LETTER A WITH BREVE */
   76|       |        // AND DOT BELOW]
   77|      1|        '\u{24B6}' => Some("A"), // Ⓐ  [CIRCLED LATIN CAPITAL LETTER A]
   78|      1|        '\u{FF21}' => Some("A"), // Ａ  [FULLWIDTH LATIN CAPITAL LETTER A]
   79|      1|        '\u{00E0}' => Some("a"), // à  [LATIN SMALL LETTER A WITH GRAVE]
   80|      1|        '\u{00E1}' => Some("a"), // á  [LATIN SMALL LETTER A WITH ACUTE]
   81|      1|        '\u{00E2}' => Some("a"), // â  [LATIN SMALL LETTER A WITH CIRCUMFLEX]
   82|      1|        '\u{00E3}' => Some("a"), // ã  [LATIN SMALL LETTER A WITH TILDE]
   83|      1|        '\u{00E4}' => Some("a"), // ä  [LATIN SMALL LETTER A WITH DIAERESIS]
   84|      1|        '\u{00E5}' => Some("a"), // å  [LATIN SMALL LETTER A WITH RING ABOVE]
   85|      1|        '\u{0101}' => Some("a"), // ā  [LATIN SMALL LETTER A WITH MACRON]
   86|      1|        '\u{0103}' => Some("a"), // ă  [LATIN SMALL LETTER A WITH BREVE]
   87|      1|        '\u{0105}' => Some("a"), // ą  [LATIN SMALL LETTER A WITH OGONEK]
   88|      1|        '\u{01CE}' => Some("a"), // ǎ  [LATIN SMALL LETTER A WITH CARON]
   89|      1|        '\u{01DF}' => Some("a"), /* ǟ  [LATIN SMALL LETTER A WITH DIAERESIS */
   90|       |        // AND MACRON]
   91|      1|        '\u{01E1}' => Some("a"), /* ǡ  [LATIN SMALL LETTER A WITH DOT ABOVE */
   92|       |        // AND MACRON]
   93|      1|        '\u{01FB}' => Some("a"), /* ǻ  [LATIN SMALL LETTER A WITH RING ABOVE */
   94|       |        // AND ACUTE]
   95|      1|        '\u{0201}' => Some("a"), // ȁ  [LATIN SMALL LETTER A WITH DOUBLE GRAVE]
   96|      1|        '\u{0203}' => Some("a"), /* ȃ  [LATIN SMALL LETTER A WITH INVERTED */
   97|       |        // BREVE]
   98|      1|        '\u{0227}' => Some("a"), // ȧ  [LATIN SMALL LETTER A WITH DOT ABOVE]
   99|      1|        '\u{0250}' => Some("a"), // ɐ  [LATIN SMALL LETTER TURNED A]
  100|      1|        '\u{0259}' => Some("a"), // ə  [LATIN SMALL LETTER SCHWA]
  101|      1|        '\u{025A}' => Some("a"), // ɚ  [LATIN SMALL LETTER SCHWA WITH HOOK]
  102|      1|        '\u{1D8F}' => Some("a"), /* ᶏ  [LATIN SMALL LETTER A WITH RETROFLEX */
  103|       |        // HOOK]
  104|      1|        '\u{1D95}' => Some("a"), /* ᶕ  [LATIN SMALL LETTER SCHWA WITH */
  105|       |        // RETROFLEX HOOK]
  106|      1|        '\u{1E01}' => Some("a"), // ạ  [LATIN SMALL LETTER A WITH RING BELOW]
  107|      1|        '\u{1E9A}' => Some("a"), /* ả  [LATIN SMALL LETTER A WITH */
  108|       |        // ALIGN_RIGHT HALF RING]
  109|      1|        '\u{1EA1}' => Some("a"), // ạ  [LATIN SMALL LETTER A WITH DOT BELOW]
  110|      1|        '\u{1EA3}' => Some("a"), // ả  [LATIN SMALL LETTER A WITH HOOK ABOVE]
  111|      1|        '\u{1EA5}' => Some("a"), /* ấ  [LATIN SMALL LETTER A WITH CIRCUMFLEX */
  112|       |        // AND ACUTE]
  113|      1|        '\u{1EA7}' => Some("a"), /* ầ  [LATIN SMALL LETTER A WITH CIRCUMFLEX */
  114|       |        // AND GRAVE]
  115|      1|        '\u{1EA9}' => Some("a"), /* ẩ  [LATIN SMALL LETTER A WITH CIRCUMFLEX */
  116|       |        // AND HOOK ABOVE]
  117|      1|        '\u{1EAB}' => Some("a"), /* ẫ  [LATIN SMALL LETTER A WITH CIRCUMFLEX */
  118|       |        // AND TILDE]
  119|      1|        '\u{1EAD}' => Some("a"), /* ậ  [LATIN SMALL LETTER A WITH CIRCUMFLEX */
  120|       |        // AND DOT BELOW]
  121|      1|        '\u{1EAF}' => Some("a"), /* ắ  [LATIN SMALL LETTER A WITH BREVE AND */
  122|       |        // ACUTE]
  123|      1|        '\u{1EB1}' => Some("a"), /* ằ  [LATIN SMALL LETTER A WITH BREVE AND */
  124|       |        // GRAVE]
  125|      1|        '\u{1EB3}' => Some("a"), /* ẳ  [LATIN SMALL LETTER A WITH BREVE AND */
  126|       |        // HOOK ABOVE]
  127|      1|        '\u{1EB5}' => Some("a"), /* ẵ  [LATIN SMALL LETTER A WITH BREVE AND */
  128|       |        // TILDE]
  129|      1|        '\u{1EB7}' => Some("a"), /* ặ  [LATIN SMALL LETTER A WITH BREVE AND */
  130|       |        // DOT BELOW]
  131|      1|        '\u{2090}' => Some("a"),  // ₐ  [LATIN SUBSCRIPT SMALL LETTER A]
  132|      1|        '\u{2094}' => Some("a"),  // ₔ  [LATIN SUBSCRIPT SMALL LETTER SCHWA]
  133|      1|        '\u{24D0}' => Some("a"),  // ⓐ  [CIRCLED LATIN SMALL LETTER A]
  134|      1|        '\u{2C65}' => Some("a"),  // ⱥ  [LATIN SMALL LETTER A WITH STROKE]
  135|      1|        '\u{2C6F}' => Some("a"),  // Ɐ  [LATIN CAPITAL LETTER TURNED A]
  136|      1|        '\u{FF41}' => Some("a"),  // ａ  [FULLWIDTH LATIN SMALL LETTER A]
  137|      1|        '\u{A732}' => Some("AA"), // Ꜳ  [LATIN CAPITAL LETTER AA]
  138|      3|        '\u{00C6}' => Some("AE"), // Æ  [LATIN CAPITAL LETTER AE]
  139|      1|        '\u{01E2}' => Some("AE"), // Ǣ  [LATIN CAPITAL LETTER AE WITH MACRON]
  140|      1|        '\u{01FC}' => Some("AE"), // Ǽ  [LATIN CAPITAL LETTER AE WITH ACUTE]
  141|      1|        '\u{1D01}' => Some("AE"), // ᴁ  [LATIN LETTER SMALL CAPITAL AE]
  142|      1|        '\u{A734}' => Some("AO"), // Ꜵ  [LATIN CAPITAL LETTER AO]
  143|      1|        '\u{A736}' => Some("AU"), // Ꜷ  [LATIN CAPITAL LETTER AU]
  144|      1|        '\u{A738}' => Some("AV"), // Ꜹ  [LATIN CAPITAL LETTER AV]
  145|      1|        '\u{A73A}' => Some("AV"), /* Ꜻ  [LATIN CAPITAL LETTER AV WITH */
  146|       |        // HORIZONTAL BAR]
  147|      1|        '\u{A73C}' => Some("AY"),  // Ꜽ  [LATIN CAPITAL LETTER AY]
  148|      1|        '\u{249C}' => Some("(a)"), // ⒜  [PARENTHESIZED LATIN SMALL LETTER A]
  149|      1|        '\u{A733}' => Some("aa"),  // ꜳ  [LATIN SMALL LETTER AA]
  150|      1|        '\u{00E6}' => Some("ae"),  // æ  [LATIN SMALL LETTER AE]
  151|      1|        '\u{01E3}' => Some("ae"),  // ǣ  [LATIN SMALL LETTER AE WITH MACRON]
  152|      1|        '\u{01FD}' => Some("ae"),  // ǽ  [LATIN SMALL LETTER AE WITH ACUTE]
  153|      1|        '\u{1D02}' => Some("ae"),  // ᴂ  [LATIN SMALL LETTER TURNED AE]
  154|      1|        '\u{A735}' => Some("ao"),  // ꜵ  [LATIN SMALL LETTER AO]
  155|      1|        '\u{A737}' => Some("au"),  // ꜷ  [LATIN SMALL LETTER AU]
  156|      1|        '\u{A739}' => Some("av"),  // ꜹ  [LATIN SMALL LETTER AV]
  157|      1|        '\u{A73B}' => Some("av"),  /* ꜻ  [LATIN SMALL LETTER AV WITH */
  158|       |        // HORIZONTAL BAR]
  159|      1|        '\u{A73D}' => Some("ay"), // ꜽ  [LATIN SMALL LETTER AY]
  160|      1|        '\u{0181}' => Some("B"),  // Ɓ  [LATIN CAPITAL LETTER B WITH HOOK]
  161|      1|        '\u{0182}' => Some("B"),  // Ƃ  [LATIN CAPITAL LETTER B WITH TOPBAR]
  162|      1|        '\u{0243}' => Some("B"),  // Ƀ  [LATIN CAPITAL LETTER B WITH STROKE]
  163|      1|        '\u{0299}' => Some("B"),  // ʙ  [LATIN LETTER SMALL CAPITAL B]
  164|      1|        '\u{1D03}' => Some("B"),  // ᴃ  [LATIN LETTER SMALL CAPITAL BARRED B]
  165|      1|        '\u{1E02}' => Some("B"),  // Ḃ  [LATIN CAPITAL LETTER B WITH DOT ABOVE]
  166|      1|        '\u{1E04}' => Some("B"),  // Ḅ  [LATIN CAPITAL LETTER B WITH DOT BELOW]
  167|      1|        '\u{1E06}' => Some("B"),  /* Ḇ  [LATIN CAPITAL LETTER B WITH LINE */
  168|       |        // BELOW]
  169|      1|        '\u{24B7}' => Some("B"),   // Ⓑ  [CIRCLED LATIN CAPITAL LETTER B]
  170|      1|        '\u{FF22}' => Some("B"),   // Ｂ  [FULLWIDTH LATIN CAPITAL LETTER B]
  171|      1|        '\u{0180}' => Some("b"),   // ƀ  [LATIN SMALL LETTER B WITH STROKE]
  172|      1|        '\u{0183}' => Some("b"),   // ƃ  [LATIN SMALL LETTER B WITH TOPBAR]
  173|      1|        '\u{0253}' => Some("b"),   // ɓ  [LATIN SMALL LETTER B WITH HOOK]
  174|      1|        '\u{1D6C}' => Some("b"),   // ᵬ  [LATIN SMALL LETTER B WITH MIDDLE TILDE]
  175|      1|        '\u{1D80}' => Some("b"),   // ᶀ  [LATIN SMALL LETTER B WITH PALATAL HOOK]
  176|      1|        '\u{1E03}' => Some("b"),   // ḃ  [LATIN SMALL LETTER B WITH DOT ABOVE]
  177|      1|        '\u{1E05}' => Some("b"),   // ḅ  [LATIN SMALL LETTER B WITH DOT BELOW]
  178|      1|        '\u{1E07}' => Some("b"),   // ḇ  [LATIN SMALL LETTER B WITH LINE BELOW]
  179|      1|        '\u{24D1}' => Some("b"),   // ⓑ  [CIRCLED LATIN SMALL LETTER B]
  180|      1|        '\u{FF42}' => Some("b"),   // ｂ  [FULLWIDTH LATIN SMALL LETTER B]
  181|      1|        '\u{249D}' => Some("(b)"), // ⒝  [PARENTHESIZED LATIN SMALL LETTER B]
  182|      1|        '\u{00C7}' => Some("C"),   // Ç  [LATIN CAPITAL LETTER C WITH CEDILLA]
  183|      1|        '\u{0106}' => Some("C"),   // Ć  [LATIN CAPITAL LETTER C WITH ACUTE]
  184|      1|        '\u{0108}' => Some("C"),   // Ĉ  [LATIN CAPITAL LETTER C WITH CIRCUMFLEX]
  185|      1|        '\u{010A}' => Some("C"),   // Ċ  [LATIN CAPITAL LETTER C WITH DOT ABOVE]
  186|      1|        '\u{010C}' => Some("C"),   // Č  [LATIN CAPITAL LETTER C WITH CARON]
  187|      1|        '\u{0187}' => Some("C"),   // Ƈ  [LATIN CAPITAL LETTER C WITH HOOK]
  188|      1|        '\u{023B}' => Some("C"),   // Ȼ  [LATIN CAPITAL LETTER C WITH STROKE]
  189|      1|        '\u{0297}' => Some("C"),   // ʗ  [LATIN LETTER STRETCHED C]
  190|      1|        '\u{1D04}' => Some("C"),   // ᴄ  [LATIN LETTER SMALL CAPITAL C]
  191|      1|        '\u{1E08}' => Some("C"),   /* Ḉ  [LATIN CAPITAL LETTER C WITH CEDILLA */
  192|       |        // AND ACUTE]
  193|      1|        '\u{24B8}' => Some("C"), // Ⓒ  [CIRCLED LATIN CAPITAL LETTER C]
  194|      1|        '\u{FF23}' => Some("C"), // Ｃ  [FULLWIDTH LATIN CAPITAL LETTER C]
  195|      1|        '\u{00E7}' => Some("c"), // ç  [LATIN SMALL LETTER C WITH CEDILLA]
  196|      1|        '\u{0107}' => Some("c"), // ć  [LATIN SMALL LETTER C WITH ACUTE]
  197|      1|        '\u{0109}' => Some("c"), // ĉ  [LATIN SMALL LETTER C WITH CIRCUMFLEX]
  198|      1|        '\u{010B}' => Some("c"), // ċ  [LATIN SMALL LETTER C WITH DOT ABOVE]
  199|      1|        '\u{010D}' => Some("c"), // č  [LATIN SMALL LETTER C WITH CARON]
  200|      1|        '\u{0188}' => Some("c"), // ƈ  [LATIN SMALL LETTER C WITH HOOK]
  201|      1|        '\u{023C}' => Some("c"), // ȼ  [LATIN SMALL LETTER C WITH STROKE]
  202|      1|        '\u{0255}' => Some("c"), // ɕ  [LATIN SMALL LETTER C WITH CURL]
  203|      1|        '\u{1E09}' => Some("c"), /* ḉ  [LATIN SMALL LETTER C WITH CEDILLA */
  204|       |        // AND ACUTE]
  205|      1|        '\u{2184}' => Some("c"), // ↄ  [LATIN SMALL LETTER REVERSED C]
  206|      1|        '\u{24D2}' => Some("c"), // ⓒ  [CIRCLED LATIN SMALL LETTER C]
  207|      1|        '\u{A73E}' => Some("c"), /* Ꜿ  [LATIN CAPITAL LETTER REVERSED C WITH */
  208|       |        // DOT]
  209|      1|        '\u{A73F}' => Some("c"),   // ꜿ  [LATIN SMALL LETTER REVERSED C WITH DOT]
  210|      1|        '\u{FF43}' => Some("c"),   // ｃ  [FULLWIDTH LATIN SMALL LETTER C]
  211|      1|        '\u{249E}' => Some("(c)"), // ⒞  [PARENTHESIZED LATIN SMALL LETTER C]
  212|      1|        '\u{00D0}' => Some("D"),   // Ð  [LATIN CAPITAL LETTER ETH]
  213|      1|        '\u{010E}' => Some("D"),   // Ď  [LATIN CAPITAL LETTER D WITH CARON]
  214|      1|        '\u{0110}' => Some("D"),   // Đ  [LATIN CAPITAL LETTER D WITH STROKE]
  215|      1|        '\u{0189}' => Some("D"),   // Ɖ  [LATIN CAPITAL LETTER AFRICAN D]
  216|      1|        '\u{018A}' => Some("D"),   // Ɗ  [LATIN CAPITAL LETTER D WITH HOOK]
  217|      1|        '\u{018B}' => Some("D"),   // Ƌ  [LATIN CAPITAL LETTER D WITH TOPBAR]
  218|      1|        '\u{1D05}' => Some("D"),   // ᴅ  [LATIN LETTER SMALL CAPITAL D]
  219|      1|        '\u{1D06}' => Some("D"),   // ᴆ  [LATIN LETTER SMALL CAPITAL ETH]
  220|      1|        '\u{1E0A}' => Some("D"),   // Ḋ  [LATIN CAPITAL LETTER D WITH DOT ABOVE]
  221|      1|        '\u{1E0C}' => Some("D"),   // Ḍ  [LATIN CAPITAL LETTER D WITH DOT BELOW]
  222|      1|        '\u{1E0E}' => Some("D"),   // Ḏ  [LATIN CAPITAL LETTER D WITH LINE BELOW]
  223|      1|        '\u{1E10}' => Some("D"),   // Ḑ  [LATIN CAPITAL LETTER D WITH CEDILLA]
  224|      1|        '\u{1E12}' => Some("D"),   /* Ḓ  [LATIN CAPITAL LETTER D WITH */
  225|       |        // CIRCUMFLEX BELOW]
  226|      1|        '\u{24B9}' => Some("D"), // Ⓓ  [CIRCLED LATIN CAPITAL LETTER D]
  227|      1|        '\u{A779}' => Some("D"), // Ꝺ  [LATIN CAPITAL LETTER INSULAR D]
  228|      1|        '\u{FF24}' => Some("D"), // Ｄ  [FULLWIDTH LATIN CAPITAL LETTER D]
  229|      1|        '\u{00F0}' => Some("d"), // ð  [LATIN SMALL LETTER ETH]
  230|      1|        '\u{010F}' => Some("d"), // ď  [LATIN SMALL LETTER D WITH CARON]
  231|      1|        '\u{0111}' => Some("d"), // đ  [LATIN SMALL LETTER D WITH STROKE]
  232|      1|        '\u{018C}' => Some("d"), // ƌ  [LATIN SMALL LETTER D WITH TOPBAR]
  233|      1|        '\u{0221}' => Some("d"), // ȡ  [LATIN SMALL LETTER D WITH CURL]
  234|      1|        '\u{0256}' => Some("d"), // ɖ  [LATIN SMALL LETTER D WITH TAIL]
  235|      1|        '\u{0257}' => Some("d"), // ɗ  [LATIN SMALL LETTER D WITH HOOK]
  236|      1|        '\u{1D6D}' => Some("d"), // ᵭ  [LATIN SMALL LETTER D WITH MIDDLE TILDE]
  237|      1|        '\u{1D81}' => Some("d"), // ᶁ  [LATIN SMALL LETTER D WITH PALATAL HOOK]
  238|      1|        '\u{1D91}' => Some("d"), /* ᶑ  [LATIN SMALL LETTER D WITH HOOK AND */
  239|       |        // TAIL]
  240|      1|        '\u{1E0B}' => Some("d"), // ḋ  [LATIN SMALL LETTER D WITH DOT ABOVE]
  241|      1|        '\u{1E0D}' => Some("d"), // ḍ  [LATIN SMALL LETTER D WITH DOT BELOW]
  242|      1|        '\u{1E0F}' => Some("d"), // ḏ  [LATIN SMALL LETTER D WITH LINE BELOW]
  243|      1|        '\u{1E11}' => Some("d"), // ḑ  [LATIN SMALL LETTER D WITH CEDILLA]
  244|      1|        '\u{1E13}' => Some("d"), /* ḓ  [LATIN SMALL LETTER D WITH CIRCUMFLEX */
  245|       |        // BELOW]
  246|      1|        '\u{24D3}' => Some("d"),  // ⓓ  [CIRCLED LATIN SMALL LETTER D]
  247|      1|        '\u{A77A}' => Some("d"),  // ꝺ  [LATIN SMALL LETTER INSULAR D]
  248|      1|        '\u{FF44}' => Some("d"),  // ｄ  [FULLWIDTH LATIN SMALL LETTER D]
  249|      1|        '\u{01C4}' => Some("DZ"), // Ǆ  [LATIN CAPITAL LETTER DZ WITH CARON]
  250|      1|        '\u{01F1}' => Some("DZ"), // Ǳ  [LATIN CAPITAL LETTER DZ]
  251|      1|        '\u{01C5}' => Some("Dz"), /* ǅ  [LATIN CAPITAL LETTER D WITH SMALL */
  252|       |        // LETTER Z WITH CARON]
  253|      1|        '\u{01F2}' => Some("Dz"), /* ǲ  [LATIN CAPITAL LETTER D WITH SMALL */
  254|       |        // LETTER Z]
  255|      1|        '\u{249F}' => Some("(d)"), // ⒟  [PARENTHESIZED LATIN SMALL LETTER D]
  256|      1|        '\u{0238}' => Some("db"),  // ȸ  [LATIN SMALL LETTER DB DIGRAPH]
  257|      1|        '\u{01C6}' => Some("dz"),  // ǆ  [LATIN SMALL LETTER DZ WITH CARON]
  258|      1|        '\u{01F3}' => Some("dz"),  // ǳ  [LATIN SMALL LETTER DZ]
  259|      1|        '\u{02A3}' => Some("dz"),  // ʣ [LATIN SMALL LETTER DZ DIGRAPH]
  260|      1|        '\u{02A5}' => Some("dz"),  /* ʥ [LATIN SMALL LETTER DZ DIGRAPH WITH */
  261|       |        // CURL]
  262|      1|        '\u{00C8}' => Some("E"), // È [LATIN CAPITAL LETTER E WITH GRAVE]
  263|      1|        '\u{00C9}' => Some("E"), // É [LATIN CAPITAL LETTER E WITH ACUTE]
  264|      1|        '\u{00CA}' => Some("E"), // Ê [LATIN CAPITAL LETTER E WITH CIRCUMFLEX]
  265|      1|        '\u{00CB}' => Some("E"), // Ë [LATIN CAPITAL LETTER E WITH DIAERESIS]
  266|      1|        '\u{0112}' => Some("E"), // Ē [LATIN CAPITAL LETTER E WITH MACRON]
  267|      1|        '\u{0114}' => Some("E"), // Ĕ [LATIN CAPITAL LETTER E WITH BREVE]
  268|      1|        '\u{0116}' => Some("E"), // Ė [LATIN CAPITAL LETTER E WITH DOT ABOVE]
  269|      1|        '\u{0118}' => Some("E"), // Ę [LATIN CAPITAL LETTER E WITH OGONEK]
  270|      1|        '\u{011A}' => Some("E"), // Ě [LATIN CAPITAL LETTER E WITH CARON]
  271|      1|        '\u{018E}' => Some("E"), // Ǝ [LATIN CAPITAL LETTER REVERSED E]
  272|      1|        '\u{0190}' => Some("E"), // Ɛ [LATIN CAPITAL LETTER OPEN E]
  273|      1|        '\u{0204}' => Some("E"), /* Ȅ [LATIN CAPITAL LETTER E WITH DOUBLE */
  274|       |        // GRAVE]
  275|      1|        '\u{0206}' => Some("E"), /* Ȇ [LATIN CAPITAL LETTER E WITH INVERTED */
  276|       |        // BREVE]
  277|      1|        '\u{0228}' => Some("E"), // Ȩ [LATIN CAPITAL LETTER E WITH CEDILLA]
  278|      1|        '\u{0246}' => Some("E"), // Ɇ [LATIN CAPITAL LETTER E WITH STROKE]
  279|      1|        '\u{1D07}' => Some("E"), // ᴇ [LATIN LETTER SMALL CAPITAL E]
  280|      1|        '\u{1E14}' => Some("E"), /* Ḕ [LATIN CAPITAL LETTER E WITH MACRON */
  281|       |        // AND GRAVE]
  282|      1|        '\u{1E16}' => Some("E"), /* Ḗ [LATIN CAPITAL LETTER E WITH MACRON */
  283|       |        // AND ACUTE]
  284|      1|        '\u{1E18}' => Some("E"), /* Ḙ [LATIN CAPITAL LETTER E WITH */
  285|       |        // CIRCUMFLEX BELOW]
  286|      1|        '\u{1E1A}' => Some("E"), // Ḛ [LATIN CAPITAL LETTER E WITH TILDE BELOW]
  287|      1|        '\u{1E1C}' => Some("E"), /* Ḝ [LATIN CAPITAL LETTER E WITH CEDILLA */
  288|       |        // AND BREVE]
  289|      1|        '\u{1EB8}' => Some("E"), // Ẹ [LATIN CAPITAL LETTER E WITH DOT BELOW]
  290|      1|        '\u{1EBA}' => Some("E"), // Ẻ [LATIN CAPITAL LETTER E WITH HOOK ABOVE]
  291|      1|        '\u{1EBC}' => Some("E"), // Ẽ [LATIN CAPITAL LETTER E WITH TILDE]
  292|      1|        '\u{1EBE}' => Some("E"), /* Ế [LATIN CAPITAL LETTER E WITH */
  293|       |        // CIRCUMFLEX AND ACUTE]
  294|      1|        '\u{1EC0}' => Some("E"), /* Ề [LATIN CAPITAL LETTER E WITH */
  295|       |        // CIRCUMFLEX AND GRAVE]
  296|      1|        '\u{1EC2}' => Some("E"), /* Ể [LATIN CAPITAL LETTER E WITH */
  297|       |        // CIRCUMFLEX AND HOOK
  298|      1|        '\u{1EC4}' => Some("E"), /* Ễ [LATIN CAPITAL LETTER E WITH */
  299|       |        // CIRCUMFLEX AND TILDE]
  300|      1|        '\u{1EC6}' => Some("E"), /* Ệ [LATIN CAPITAL LETTER E WITH */
  301|       |        // CIRCUMFLEX AND DOT BELOW]
  302|      1|        '\u{24BA}' => Some("E"), // Ⓔ [CIRCLED LATIN CAPITAL LETTER E]
  303|      1|        '\u{2C7B}' => Some("E"), // ⱻ [LATIN LETTER SMALL CAPITAL TURNED E]
  304|      1|        '\u{FF25}' => Some("E"), // Ｅ [FULLWIDTH LATIN CAPITAL LETTER E]
  305|      1|        '\u{00E8}' => Some("e"), // è [LATIN SMALL LETTER E WITH GRAVE]
  306|      1|        '\u{00E9}' => Some("e"), // é [LATIN SMALL LETTER E WITH ACUTE]
  307|      1|        '\u{00EA}' => Some("e"), // ê [LATIN SMALL LETTER E WITH CIRCUMFLEX]
  308|      1|        '\u{00EB}' => Some("e"), // ë [LATIN SMALL LETTER E WITH DIAERESIS]
  309|      1|        '\u{0113}' => Some("e"), // ē [LATIN SMALL LETTER E WITH MACRON]
  310|      1|        '\u{0115}' => Some("e"), // ĕ [LATIN SMALL LETTER E WITH BREVE]
  311|      1|        '\u{0117}' => Some("e"), // ė [LATIN SMALL LETTER E WITH DOT ABOVE]
  312|      1|        '\u{0119}' => Some("e"), // ę [LATIN SMALL LETTER E WITH OGONEK]
  313|      1|        '\u{011B}' => Some("e"), // ě [LATIN SMALL LETTER E WITH CARON]
  314|      1|        '\u{01DD}' => Some("e"), // ǝ [LATIN SMALL LETTER TURNED E]
  315|      1|        '\u{0205}' => Some("e"), // ȅ [LATIN SMALL LETTER E WITH DOUBLE GRAVE]
  316|      1|        '\u{0207}' => Some("e"), /* ȇ [LATIN SMALL LETTER E WITH INVERTED */
  317|       |        // BREVE]
  318|      1|        '\u{0229}' => Some("e"), // ȩ [LATIN SMALL LETTER E WITH CEDILLA]
  319|      1|        '\u{0247}' => Some("e"), // ɇ [LATIN SMALL LETTER E WITH STROKE]
  320|      1|        '\u{0258}' => Some("e"), // ɘ [LATIN SMALL LETTER REVERSED E]
  321|      1|        '\u{025B}' => Some("e"), // ɛ [LATIN SMALL LETTER OPEN E]
  322|      1|        '\u{025C}' => Some("e"), // ɜ [LATIN SMALL LETTER REVERSED OPEN E]
  323|      1|        '\u{025D}' => Some("e"), /* ɝ [LATIN SMALL LETTER REVERSED OPEN E */
  324|       |        // WITH HOOK]
  325|      1|        '\u{025E}' => Some("e"), /* ɞ [LATIN SMALL LETTER CLOSED REVERSED */
  326|       |        // OPEN E]
  327|      1|        '\u{029A}' => Some("e"), // ʚ [LATIN SMALL LETTER CLOSED OPEN E]
  328|      1|        '\u{1D08}' => Some("e"), // ᴈ [LATIN SMALL LETTER TURNED OPEN E]
  329|      1|        '\u{1D92}' => Some("e"), /* ᶒ [LATIN SMALL LETTER E WITH RETROFLEX */
  330|       |        // HOOK]
  331|      1|        '\u{1D93}' => Some("e"), /* ᶓ [LATIN SMALL LETTER OPEN E WITH */
  332|       |        // RETROFLEX HOOK]
  333|      1|        '\u{1D94}' => Some("e"), /* ᶔ [LATIN SMALL LETTER REVERSED OPEN E */
  334|       |        // WITH RETROFLEX HOOK]
  335|      1|        '\u{1E15}' => Some("e"), /* ḕ [LATIN SMALL LETTER E WITH MACRON AND */
  336|       |        // GRAVE]
  337|      1|        '\u{1E17}' => Some("e"), /* ḗ [LATIN SMALL LETTER E WITH MACRON AND */
  338|       |        // ACUTE]
  339|      1|        '\u{1E19}' => Some("e"), /* ḙ [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  340|       |        // BELOW]
  341|      1|        '\u{1E1B}' => Some("e"), // ḛ [LATIN SMALL LETTER E WITH TILDE BELOW]
  342|      1|        '\u{1E1D}' => Some("e"), /* ḝ [LATIN SMALL LETTER E WITH CEDILLA AND */
  343|       |        // BREVE]
  344|      1|        '\u{1EB9}' => Some("e"), // ẹ [LATIN SMALL LETTER E WITH DOT BELOW]
  345|      1|        '\u{1EBB}' => Some("e"), // ẻ [LATIN SMALL LETTER E WITH HOOK ABOVE]
  346|      1|        '\u{1EBD}' => Some("e"), // ẽ [LATIN SMALL LETTER E WITH TILDE]
  347|      1|        '\u{1EBF}' => Some("e"), /* ế [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  348|       |        // AND ACUTE]
  349|      1|        '\u{1EC1}' => Some("e"), /* ề [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  350|       |        // AND GRAVE]
  351|      1|        '\u{1EC3}' => Some("e"), /* ể [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  352|       |        // AND HOOK ABOVE]
  353|      1|        '\u{1EC5}' => Some("e"), /* ễ [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  354|       |        // AND TILDE]
  355|      1|        '\u{1EC7}' => Some("e"), /* ệ [LATIN SMALL LETTER E WITH CIRCUMFLEX */
  356|       |        // AND DOT BELOW]
  357|      1|        '\u{2091}' => Some("e"),   // ₑ [LATIN SUBSCRIPT SMALL LETTER E]
  358|      1|        '\u{24D4}' => Some("e"),   // ⓔ [CIRCLED LATIN SMALL LETTER E]
  359|      1|        '\u{2C78}' => Some("e"),   // ⱸ [LATIN SMALL LETTER E WITH NOTCH]
  360|      1|        '\u{FF45}' => Some("e"),   // ｅ [FULLWIDTH LATIN SMALL LETTER E]
  361|      1|        '\u{24A0}' => Some("(e)"), // ⒠ [PARENTHESIZED LATIN SMALL LETTER E]
  362|      1|        '\u{0191}' => Some("F"),   // Ƒ [LATIN CAPITAL LETTER F WITH HOOK]
  363|      1|        '\u{1E1E}' => Some("F"),   // Ḟ [LATIN CAPITAL LETTER F WITH DOT ABOVE]
  364|      1|        '\u{24BB}' => Some("F"),   // Ⓕ [CIRCLED LATIN CAPITAL LETTER F]
  365|      1|        '\u{A730}' => Some("F"),   // ꜰ [LATIN LETTER SMALL CAPITAL F]
  366|      1|        '\u{A77B}' => Some("F"),   // Ꝼ [LATIN CAPITAL LETTER INSULAR F]
  367|      1|        '\u{A7FB}' => Some("F"),   // ꟻ [LATIN EPIGRAPHIC LETTER REVERSED F]
  368|      1|        '\u{FF26}' => Some("F"),   // Ｆ [FULLWIDTH LATIN CAPITAL LETTER F]
  369|      1|        '\u{0192}' => Some("f"),   // ƒ [LATIN SMALL LETTER F WITH HOOK]
  370|      1|        '\u{1D6E}' => Some("f"),   // ᵮ [LATIN SMALL LETTER F WITH MIDDLE TILDE]
  371|      1|        '\u{1D82}' => Some("f"),   // ᶂ [LATIN SMALL LETTER F WITH PALATAL HOOK]
  372|      1|        '\u{1E1F}' => Some("f"),   // ḟ [LATIN SMALL LETTER F WITH DOT ABOVE]
  373|      1|        '\u{1E9B}' => Some("f"),   /* ẛ [LATIN SMALL LETTER LONG S WITH DOT */
  374|       |        // ABOVE]
  375|      1|        '\u{24D5}' => Some("f"),   // ⓕ [CIRCLED LATIN SMALL LETTER F]
  376|      1|        '\u{A77C}' => Some("f"),   // ꝼ [LATIN SMALL LETTER INSULAR F]
  377|      1|        '\u{FF46}' => Some("f"),   // ｆ [FULLWIDTH LATIN SMALL LETTER F]
  378|      1|        '\u{24A1}' => Some("(f)"), // ⒡ [PARENTHESIZED LATIN SMALL LETTER F]
  379|      1|        '\u{FB00}' => Some("ff"),  // ﬀ [LATIN SMALL LIGATURE FF]
  380|      1|        '\u{FB03}' => Some("ffi"), // ﬃ [LATIN SMALL LIGATURE FFI]
  381|      1|        '\u{FB04}' => Some("ffl"), // ﬄ [LATIN SMALL LIGATURE FFL]
  382|      1|        '\u{FB01}' => Some("fi"),  // ﬁ [LATIN SMALL LIGATURE FI]
  383|      1|        '\u{FB02}' => Some("fl"),  // ﬂ [LATIN SMALL LIGATURE FL]
  384|      1|        '\u{011C}' => Some("G"),   // Ĝ [LATIN CAPITAL LETTER G WITH CIRCUMFLEX]
  385|      1|        '\u{011E}' => Some("G"),   // Ğ [LATIN CAPITAL LETTER G WITH BREVE]
  386|      1|        '\u{0120}' => Some("G"),   // Ġ [LATIN CAPITAL LETTER G WITH DOT ABOVE]
  387|      1|        '\u{0122}' => Some("G"),   // Ģ [LATIN CAPITAL LETTER G WITH CEDILLA]
  388|      1|        '\u{0193}' => Some("G"),   // Ɠ [LATIN CAPITAL LETTER G WITH HOOK]
  389|      1|        '\u{01E4}' => Some("G"),   // Ǥ [LATIN CAPITAL LETTER G WITH STROKE]
  390|      1|        '\u{01E5}' => Some("G"),   // ǥ [LATIN SMALL LETTER G WITH STROKE]
  391|      1|        '\u{01E6}' => Some("G"),   // Ǧ [LATIN CAPITAL LETTER G WITH CARON]
  392|      1|        '\u{01E7}' => Some("G"),   // ǧ [LATIN SMALL LETTER G WITH CARON]
  393|      1|        '\u{01F4}' => Some("G"),   // Ǵ [LATIN CAPITAL LETTER G WITH ACUTE]
  394|      1|        '\u{0262}' => Some("G"),   // ɢ [LATIN LETTER SMALL CAPITAL G]
  395|      1|        '\u{029B}' => Some("G"),   // ʛ [LATIN LETTER SMALL CAPITAL G WITH HOOK]
  396|      1|        '\u{1E20}' => Some("G"),   // Ḡ [LATIN CAPITAL LETTER G WITH MACRON]
  397|      1|        '\u{24BC}' => Some("G"),   // Ⓖ [CIRCLED LATIN CAPITAL LETTER G]
  398|      1|        '\u{A77D}' => Some("G"),   // Ᵹ [LATIN CAPITAL LETTER INSULAR G]
  399|      1|        '\u{A77E}' => Some("G"),   // Ꝿ [LATIN CAPITAL LETTER TURNED INSULAR G]
  400|      1|        '\u{FF27}' => Some("G"),   // Ｇ [FULLWIDTH LATIN CAPITAL LETTER G]
  401|      1|        '\u{011D}' => Some("g"),   // ĝ [LATIN SMALL LETTER G WITH CIRCUMFLEX]
  402|      1|        '\u{011F}' => Some("g"),   // ğ [LATIN SMALL LETTER G WITH BREVE]
  403|      1|        '\u{0121}' => Some("g"),   // ġ [LATIN SMALL LETTER G WITH DOT ABOVE]
  404|      1|        '\u{0123}' => Some("g"),   // ģ [LATIN SMALL LETTER G WITH CEDILLA]
  405|      1|        '\u{01F5}' => Some("g"),   // ǵ [LATIN SMALL LETTER G WITH ACUTE]
  406|      1|        '\u{0260}' => Some("g"),   // ɠ [LATIN SMALL LETTER G WITH HOOK]
  407|      1|        '\u{0261}' => Some("g"),   // ɡ [LATIN SMALL LETTER SCRIPT G]
  408|      1|        '\u{1D77}' => Some("g"),   // ᵷ [LATIN SMALL LETTER TURNED G]
  409|      1|        '\u{1D79}' => Some("g"),   // ᵹ [LATIN SMALL LETTER INSULAR G]
  410|      1|        '\u{1D83}' => Some("g"),   // ᶃ [LATIN SMALL LETTER G WITH PALATAL HOOK]
  411|      1|        '\u{1E21}' => Some("g"),   // ḡ [LATIN SMALL LETTER G WITH MACRON]
  412|      1|        '\u{24D6}' => Some("g"),   // ⓖ [CIRCLED LATIN SMALL LETTER G]
  413|      1|        '\u{A77F}' => Some("g"),   // ꝿ [LATIN SMALL LETTER TURNED INSULAR G]
  414|      1|        '\u{FF47}' => Some("g"),   // ｇ [FULLWIDTH LATIN SMALL LETTER G]
  415|      1|        '\u{24A2}' => Some("(g)"), // ⒢ [PARENTHESIZED LATIN SMALL LETTER G]
  416|      1|        '\u{0124}' => Some("H"),   // Ĥ [LATIN CAPITAL LETTER H WITH CIRCUMFLEX]
  417|      1|        '\u{0126}' => Some("H"),   // Ħ [LATIN CAPITAL LETTER H WITH STROKE]
  418|      1|        '\u{021E}' => Some("H"),   // Ȟ [LATIN CAPITAL LETTER H WITH CARON]
  419|      1|        '\u{029C}' => Some("H"),   // ʜ [LATIN LETTER SMALL CAPITAL H]
  420|      1|        '\u{1E22}' => Some("H"),   // Ḣ [LATIN CAPITAL LETTER H WITH DOT ABOVE]
  421|      1|        '\u{1E24}' => Some("H"),   // Ḥ [LATIN CAPITAL LETTER H WITH DOT BELOW]
  422|      1|        '\u{1E26}' => Some("H"),   // Ḧ [LATIN CAPITAL LETTER H WITH DIAERESIS]
  423|      1|        '\u{1E28}' => Some("H"),   // Ḩ [LATIN CAPITAL LETTER H WITH CEDILLA]
  424|      1|        '\u{1E2A}' => Some("H"),   // Ḫ [LATIN CAPITAL LETTER H WITH BREVE BELOW]
  425|      1|        '\u{24BD}' => Some("H"),   // Ⓗ [CIRCLED LATIN CAPITAL LETTER H]
  426|      1|        '\u{2C67}' => Some("H"),   // Ⱨ [LATIN CAPITAL LETTER H WITH DESCENDER]
  427|      1|        '\u{2C75}' => Some("H"),   // Ⱶ [LATIN CAPITAL LETTER HALF H]
  428|      1|        '\u{FF28}' => Some("H"),   // Ｈ [FULLWIDTH LATIN CAPITAL LETTER H]
  429|      1|        '\u{0125}' => Some("h"),   // ĥ [LATIN SMALL LETTER H WITH CIRCUMFLEX]
  430|      1|        '\u{0127}' => Some("h"),   // ħ [LATIN SMALL LETTER H WITH STROKE]
  431|      1|        '\u{021F}' => Some("h"),   // ȟ [LATIN SMALL LETTER H WITH CARON]
  432|      1|        '\u{0265}' => Some("h"),   // ɥ [LATIN SMALL LETTER TURNED H]
  433|      1|        '\u{0266}' => Some("h"),   // ɦ [LATIN SMALL LETTER H WITH HOOK]
  434|      1|        '\u{02AE}' => Some("h"),   /* ʮ [LATIN SMALL LETTER TURNED H WITH */
  435|       |        // FISHHOOK]
  436|      1|        '\u{02AF}' => Some("h"), /* ʯ [LATIN SMALL LETTER TURNED H WITH */
  437|       |        // FISHHOOK AND TAIL]
  438|      1|        '\u{1E23}' => Some("h"),   // ḣ [LATIN SMALL LETTER H WITH DOT ABOVE]
  439|      1|        '\u{1E25}' => Some("h"),   // ḥ [LATIN SMALL LETTER H WITH DOT BELOW]
  440|      1|        '\u{1E27}' => Some("h"),   // ḧ [LATIN SMALL LETTER H WITH DIAERESIS]
  441|      1|        '\u{1E29}' => Some("h"),   // ḩ [LATIN SMALL LETTER H WITH CEDILLA]
  442|      1|        '\u{1E2B}' => Some("h"),   // ḫ [LATIN SMALL LETTER H WITH BREVE BELOW]
  443|      1|        '\u{1E96}' => Some("h"),   // ẖ [LATIN SMALL LETTER H WITH LINE BELOW]
  444|      1|        '\u{24D7}' => Some("h"),   // ⓗ [CIRCLED LATIN SMALL LETTER H]
  445|      1|        '\u{2C68}' => Some("h"),   // ⱨ [LATIN SMALL LETTER H WITH DESCENDER]
  446|      1|        '\u{2C76}' => Some("h"),   // ⱶ [LATIN SMALL LETTER HALF H]
  447|      1|        '\u{FF48}' => Some("h"),   // ｈ [FULLWIDTH LATIN SMALL LETTER H]
  448|      1|        '\u{01F6}' => Some("HV"),  /* Ƕ http://en.wikipedia.org/wiki/Hwair [LATIN CAPITAL LETTER HWAIR] */
  449|      1|        '\u{24A3}' => Some("(h)"), // ⒣ [PARENTHESIZED LATIN SMALL LETTER H]
  450|      1|        '\u{0195}' => Some("hv"),  // ƕ [LATIN SMALL LETTER HV]
  451|      1|        '\u{00CC}' => Some("I"),   // Ì [LATIN CAPITAL LETTER I WITH GRAVE]
  452|      1|        '\u{00CD}' => Some("I"),   // Í [LATIN CAPITAL LETTER I WITH ACUTE]
  453|      1|        '\u{00CE}' => Some("I"),   // Î [LATIN CAPITAL LETTER I WITH CIRCUMFLEX]
  454|      1|        '\u{00CF}' => Some("I"),   // Ï [LATIN CAPITAL LETTER I WITH DIAERESIS]
  455|      1|        '\u{0128}' => Some("I"),   // Ĩ [LATIN CAPITAL LETTER I WITH TILDE]
  456|      1|        '\u{012A}' => Some("I"),   // Ī [LATIN CAPITAL LETTER I WITH MACRON]
  457|      1|        '\u{012C}' => Some("I"),   // Ĭ [LATIN CAPITAL LETTER I WITH BREVE]
  458|      1|        '\u{012E}' => Some("I"),   // Į [LATIN CAPITAL LETTER I WITH OGONEK]
  459|      1|        '\u{0130}' => Some("I"),   // İ [LATIN CAPITAL LETTER I WITH DOT ABOVE]
  460|      1|        '\u{0196}' => Some("I"),   // Ɩ [LATIN CAPITAL LETTER IOTA]
  461|      1|        '\u{0197}' => Some("I"),   // Ɨ [LATIN CAPITAL LETTER I WITH STROKE]
  462|      1|        '\u{01CF}' => Some("I"),   // Ǐ [LATIN CAPITAL LETTER I WITH CARON]
  463|      1|        '\u{0208}' => Some("I"),   /* Ȉ [LATIN CAPITAL LETTER I WITH DOUBLE */
  464|       |        // GRAVE]
  465|      1|        '\u{020A}' => Some("I"), /* Ȋ [LATIN CAPITAL LETTER I WITH INVERTED */
  466|       |        // BREVE]
  467|      1|        '\u{026A}' => Some("I"), // ɪ [LATIN LETTER SMALL CAPITAL I]
  468|      1|        '\u{1D7B}' => Some("I"), /* ᵻ [LATIN SMALL CAPITAL LETTER I WITH */
  469|       |        // STROKE]
  470|      1|        '\u{1E2C}' => Some("I"), // Ḭ [LATIN CAPITAL LETTER I WITH TILDE BELOW]
  471|      1|        '\u{1E2E}' => Some("I"), /* Ḯ [LATIN CAPITAL LETTER I WITH DIAERESIS */
  472|       |        // AND ACUTE]
  473|      1|        '\u{1EC8}' => Some("I"), // Ỉ [LATIN CAPITAL LETTER I WITH HOOK ABOVE]
  474|      1|        '\u{1ECA}' => Some("I"), // Ị [LATIN CAPITAL LETTER I WITH DOT BELOW]
  475|      1|        '\u{24BE}' => Some("I"), // Ⓘ [CIRCLED LATIN CAPITAL LETTER I]
  476|      1|        '\u{A7FE}' => Some("I"), // ꟾ [LATIN EPIGRAPHIC LETTER I LONGA]
  477|      1|        '\u{FF29}' => Some("I"), // Ｉ [FULLWIDTH LATIN CAPITAL LETTER I]
  478|      1|        '\u{00EC}' => Some("i"), // ì [LATIN SMALL LETTER I WITH GRAVE]
  479|      1|        '\u{00ED}' => Some("i"), // í [LATIN SMALL LETTER I WITH ACUTE]
  480|      1|        '\u{00EE}' => Some("i"), // î [LATIN SMALL LETTER I WITH CIRCUMFLEX]
  481|      1|        '\u{00EF}' => Some("i"), // ï [LATIN SMALL LETTER I WITH DIAERESIS]
  482|      1|        '\u{0129}' => Some("i"), // ĩ [LATIN SMALL LETTER I WITH TILDE]
  483|      1|        '\u{012B}' => Some("i"), // ī [LATIN SMALL LETTER I WITH MACRON]
  484|      1|        '\u{012D}' => Some("i"), // ĭ [LATIN SMALL LETTER I WITH BREVE]
  485|      1|        '\u{012F}' => Some("i"), // į [LATIN SMALL LETTER I WITH OGONEK]
  486|      1|        '\u{0131}' => Some("i"), // ı [LATIN SMALL LETTER DOTLESS I]
  487|      1|        '\u{01D0}' => Some("i"), // ǐ [LATIN SMALL LETTER I WITH CARON]
  488|      1|        '\u{0209}' => Some("i"), // ȉ [LATIN SMALL LETTER I WITH DOUBLE GRAVE]
  489|      1|        '\u{020B}' => Some("i"), /* ȋ [LATIN SMALL LETTER I WITH INVERTED */
  490|       |        // BREVE]
  491|      1|        '\u{0268}' => Some("i"), // ɨ [LATIN SMALL LETTER I WITH STROKE]
  492|      1|        '\u{1D09}' => Some("i"), // ᴉ [LATIN SMALL LETTER TURNED I]
  493|      1|        '\u{1D62}' => Some("i"), // ᵢ [LATIN SUBSCRIPT SMALL LETTER I]
  494|      1|        '\u{1D7C}' => Some("i"), // ᵼ [LATIN SMALL LETTER IOTA WITH STROKE]
  495|      1|        '\u{1D96}' => Some("i"), /* ᶖ [LATIN SMALL LETTER I WITH RETROFLEX */
  496|       |        // HOOK]
  497|      1|        '\u{1E2D}' => Some("i"), // ḭ [LATIN SMALL LETTER I WITH TILDE BELOW]
  498|      1|        '\u{1E2F}' => Some("i"), /* ḯ [LATIN SMALL LETTER I WITH DIAERESIS */
  499|       |        // AND ACUTE]
  500|      1|        '\u{1EC9}' => Some("i"),   // ỉ [LATIN SMALL LETTER I WITH HOOK ABOVE]
  501|      1|        '\u{1ECB}' => Some("i"),   // ị [LATIN SMALL LETTER I WITH DOT BELOW]
  502|      1|        '\u{2071}' => Some("i"),   // ⁱ [SUPERSCRIPT LATIN SMALL LETTER I]
  503|      1|        '\u{24D8}' => Some("i"),   // ⓘ [CIRCLED LATIN SMALL LETTER I]
  504|      1|        '\u{FF49}' => Some("i"),   // ｉ [FULLWIDTH LATIN SMALL LETTER I]
  505|      1|        '\u{0132}' => Some("IJ"),  // Ĳ [LATIN CAPITAL LIGATURE IJ]
  506|      1|        '\u{24A4}' => Some("(i)"), // ⒤ [PARENTHESIZED LATIN SMALL LETTER I]
  507|      1|        '\u{0133}' => Some("ij"),  // ĳ [LATIN SMALL LIGATURE IJ]
  508|      1|        '\u{0134}' => Some("J"),   // Ĵ [LATIN CAPITAL LETTER J WITH CIRCUMFLEX]
  509|      1|        '\u{0248}' => Some("J"),   // Ɉ [LATIN CAPITAL LETTER J WITH STROKE]
  510|      1|        '\u{1D0A}' => Some("J"),   // ᴊ [LATIN LETTER SMALL CAPITAL J]
  511|      1|        '\u{24BF}' => Some("J"),   // Ⓙ [CIRCLED LATIN CAPITAL LETTER J]
  512|      1|        '\u{FF2A}' => Some("J"),   // Ｊ [FULLWIDTH LATIN CAPITAL LETTER J]
  513|      1|        '\u{0135}' => Some("j"),   // ĵ [LATIN SMALL LETTER J WITH CIRCUMFLEX]
  514|      1|        '\u{01F0}' => Some("j"),   // ǰ [LATIN SMALL LETTER J WITH CARON]
  515|      1|        '\u{0237}' => Some("j"),   // ȷ [LATIN SMALL LETTER DOTLESS J]
  516|      1|        '\u{0249}' => Some("j"),   // ɉ [LATIN SMALL LETTER J WITH STROKE]
  517|      1|        '\u{025F}' => Some("j"),   /* ɟ [LATIN SMALL LETTER DOTLESS J WITH */
  518|       |        // STROKE]
  519|      1|        '\u{0284}' => Some("j"), /* ʄ [LATIN SMALL LETTER DOTLESS J WITH */
  520|       |        // STROKE AND HOOK]
  521|      1|        '\u{029D}' => Some("j"),   // ʝ [LATIN SMALL LETTER J WITH CROSSED-TAIL]
  522|      1|        '\u{24D9}' => Some("j"),   // ⓙ [CIRCLED LATIN SMALL LETTER J]
  523|      1|        '\u{2C7C}' => Some("j"),   // ⱼ [LATIN SUBSCRIPT SMALL LETTER J]
  524|      1|        '\u{FF4A}' => Some("j"),   // ｊ [FULLWIDTH LATIN SMALL LETTER J]
  525|      1|        '\u{24A5}' => Some("(j)"), // ⒥ [PARENTHESIZED LATIN SMALL LETTER J]
  526|      1|        '\u{0136}' => Some("K"),   // Ķ [LATIN CAPITAL LETTER K WITH CEDILLA]
  527|      1|        '\u{0198}' => Some("K"),   // Ƙ [LATIN CAPITAL LETTER K WITH HOOK]
  528|      1|        '\u{01E8}' => Some("K"),   // Ǩ [LATIN CAPITAL LETTER K WITH CARON]
  529|      1|        '\u{1D0B}' => Some("K"),   // ᴋ [LATIN LETTER SMALL CAPITAL K]
  530|      1|        '\u{1E30}' => Some("K"),   // Ḱ [LATIN CAPITAL LETTER K WITH ACUTE]
  531|      1|        '\u{1E32}' => Some("K"),   // Ḳ [LATIN CAPITAL LETTER K WITH DOT BELOW]
  532|      1|        '\u{1E34}' => Some("K"),   // Ḵ [LATIN CAPITAL LETTER K WITH LINE BELOW]
  533|      1|        '\u{24C0}' => Some("K"),   // Ⓚ [CIRCLED LATIN CAPITAL LETTER K]
  534|      1|        '\u{2C69}' => Some("K"),   // Ⱪ [LATIN CAPITAL LETTER K WITH DESCENDER]
  535|      1|        '\u{A740}' => Some("K"),   // Ꝁ [LATIN CAPITAL LETTER K WITH STROKE]
  536|      1|        '\u{A742}' => Some("K"),   /* Ꝃ [LATIN CAPITAL LETTER K WITH DIAGONAL */
  537|       |        // STROKE]
  538|      1|        '\u{A744}' => Some("K"), /* Ꝅ [LATIN CAPITAL LETTER K WITH STROKE */
  539|       |        // AND DIAGONAL
  540|      1|        '\u{FF2B}' => Some("K"), // Ｋ [FULLWIDTH LATIN CAPITAL LETTER K]
  541|      1|        '\u{0137}' => Some("k"), // ķ [LATIN SMALL LETTER K WITH CEDILLA]
  542|      1|        '\u{0199}' => Some("k"), // ƙ [LATIN SMALL LETTER K WITH HOOK]
  543|      1|        '\u{01E9}' => Some("k"), // ǩ [LATIN SMALL LETTER K WITH CARON]
  544|      1|        '\u{029E}' => Some("k"), // ʞ [LATIN SMALL LETTER TURNED K]
  545|      1|        '\u{1D84}' => Some("k"), // ᶄ [LATIN SMALL LETTER K WITH PALATAL HOOK]
  546|      1|        '\u{1E31}' => Some("k"), // ḱ [LATIN SMALL LETTER K WITH ACUTE]
  547|      1|        '\u{1E33}' => Some("k"), // ḳ [LATIN SMALL LETTER K WITH DOT BELOW]
  548|      1|        '\u{1E35}' => Some("k"), // ḵ [LATIN SMALL LETTER K WITH LINE BELOW]
  549|      1|        '\u{24DA}' => Some("k"), // ⓚ [CIRCLED LATIN SMALL LETTER K]
  550|      1|        '\u{2C6A}' => Some("k"), // ⱪ [LATIN SMALL LETTER K WITH DESCENDER]
  551|      1|        '\u{A741}' => Some("k"), // ꝁ [LATIN SMALL LETTER K WITH STROKE]
  552|      1|        '\u{A743}' => Some("k"), /* ꝃ [LATIN SMALL LETTER K WITH DIAGONAL */
  553|       |        // STROKE]
  554|      1|        '\u{A745}' => Some("k"), /* ꝅ [LATIN SMALL LETTER K WITH STROKE AND */
  555|       |        // DIAGONAL STROKE]
  556|      1|        '\u{FF4B}' => Some("k"),   // ｋ [FULLWIDTH LATIN SMALL LETTER K]
  557|      1|        '\u{24A6}' => Some("(k)"), // ⒦ [PARENTHESIZED LATIN SMALL LETTER K]
  558|      1|        '\u{0139}' => Some("L"),   // Ĺ [LATIN CAPITAL LETTER L WITH ACUTE]
  559|      1|        '\u{013B}' => Some("L"),   // Ļ [LATIN CAPITAL LETTER L WITH CEDILLA]
  560|      1|        '\u{013D}' => Some("L"),   // Ľ [LATIN CAPITAL LETTER L WITH CARON]
  561|      1|        '\u{013F}' => Some("L"),   // Ŀ [LATIN CAPITAL LETTER L WITH MIDDLE DOT]
  562|      1|        '\u{0141}' => Some("L"),   // Ł [LATIN CAPITAL LETTER L WITH STROKE]
  563|      1|        '\u{023D}' => Some("L"),   // Ƚ [LATIN CAPITAL LETTER L WITH BAR]
  564|      1|        '\u{029F}' => Some("L"),   // ʟ [LATIN LETTER SMALL CAPITAL L]
  565|      1|        '\u{1D0C}' => Some("L"),   /* ᴌ [LATIN LETTER SMALL CAPITAL L WITH */
  566|       |        // STROKE]
  567|      1|        '\u{1E36}' => Some("L"), // Ḷ [LATIN CAPITAL LETTER L WITH DOT BELOW]
  568|      1|        '\u{1E38}' => Some("L"), /* Ḹ [LATIN CAPITAL LETTER L WITH DOT BELOW */
  569|       |        // AND MACRON]
  570|      1|        '\u{1E3A}' => Some("L"), // Ḻ [LATIN CAPITAL LETTER L WITH LINE BELOW]
  571|      1|        '\u{1E3C}' => Some("L"), /* Ḽ [LATIN CAPITAL LETTER L WITH */
  572|       |        // CIRCUMFLEX BELOW]
  573|      1|        '\u{24C1}' => Some("L"), // Ⓛ [CIRCLED LATIN CAPITAL LETTER L]
  574|      1|        '\u{2C60}' => Some("L"), // Ⱡ [LATIN CAPITAL LETTER L WITH DOUBLE BAR]
  575|      1|        '\u{2C62}' => Some("L"), /* Ɫ [LATIN CAPITAL LETTER L WITH MIDDLE */
  576|       |        // TILDE]
  577|      1|        '\u{A746}' => Some("L"), // Ꝇ [LATIN CAPITAL LETTER BROKEN L]
  578|      1|        '\u{A748}' => Some("L"), // Ꝉ [LATIN CAPITAL LETTER L WITH HIGH STROKE]
  579|      1|        '\u{A780}' => Some("L"), // Ꞁ [LATIN CAPITAL LETTER TURNED L]
  580|      1|        '\u{FF2C}' => Some("L"), // Ｌ [FULLWIDTH LATIN CAPITAL LETTER L]
  581|      1|        '\u{013A}' => Some("l"), // ĺ [LATIN SMALL LETTER L WITH ACUTE]
  582|      1|        '\u{013C}' => Some("l"), // ļ [LATIN SMALL LETTER L WITH CEDILLA]
  583|      1|        '\u{013E}' => Some("l"), // ľ [LATIN SMALL LETTER L WITH CARON]
  584|      1|        '\u{0140}' => Some("l"), // ŀ [LATIN SMALL LETTER L WITH MIDDLE DOT]
  585|      1|        '\u{0142}' => Some("l"), // ł [LATIN SMALL LETTER L WITH STROKE]
  586|      1|        '\u{019A}' => Some("l"), // ƚ [LATIN SMALL LETTER L WITH BAR]
  587|      1|        '\u{0234}' => Some("l"), // ȴ [LATIN SMALL LETTER L WITH CURL]
  588|      1|        '\u{026B}' => Some("l"), // ɫ [LATIN SMALL LETTER L WITH MIDDLE TILDE]
  589|      1|        '\u{026C}' => Some("l"), // ɬ [LATIN SMALL LETTER L WITH BELT]
  590|      1|        '\u{026D}' => Some("l"), /* ɭ [LATIN SMALL LETTER L WITH RETROFLEX */
  591|       |        // HOOK]
  592|      1|        '\u{1D85}' => Some("l"), // ᶅ [LATIN SMALL LETTER L WITH PALATAL HOOK]
  593|      1|        '\u{1E37}' => Some("l"), // ḷ [LATIN SMALL LETTER L WITH DOT BELOW]
  594|      1|        '\u{1E39}' => Some("l"), /* ḹ [LATIN SMALL LETTER L WITH DOT BELOW */
  595|       |        // AND MACRON]
  596|      1|        '\u{1E3B}' => Some("l"), // ḻ [LATIN SMALL LETTER L WITH LINE BELOW]
  597|      1|        '\u{1E3D}' => Some("l"), /* ḽ [LATIN SMALL LETTER L WITH CIRCUMFLEX */
  598|       |        // BELOW]
  599|      1|        '\u{24DB}' => Some("l"),  // ⓛ [CIRCLED LATIN SMALL LETTER L]
  600|      1|        '\u{2C61}' => Some("l"),  // ⱡ [LATIN SMALL LETTER L WITH DOUBLE BAR]
  601|      1|        '\u{A747}' => Some("l"),  // ꝇ [LATIN SMALL LETTER BROKEN L]
  602|      1|        '\u{A749}' => Some("l"),  // ꝉ [LATIN SMALL LETTER L WITH HIGH STROKE]
  603|      1|        '\u{A781}' => Some("l"),  // ꞁ [LATIN SMALL LETTER TURNED L]
  604|      1|        '\u{FF4C}' => Some("l"),  // ｌ [FULLWIDTH LATIN SMALL LETTER L]
  605|      1|        '\u{01C7}' => Some("LJ"), // Ǉ [LATIN CAPITAL LETTER LJ]
  606|      1|        '\u{1EFA}' => Some("LL"), // Ỻ [LATIN CAPITAL LETTER MIDDLE-WELSH LL]
  607|      1|        '\u{01C8}' => Some("Lj"), /* ǈ [LATIN CAPITAL LETTER L WITH SMALL */
  608|       |        // LETTER J]
  609|      1|        '\u{24A7}' => Some("(l)"), // ⒧ [PARENTHESIZED LATIN SMALL LETTER L]
  610|      1|        '\u{01C9}' => Some("lj"),  // ǉ [LATIN SMALL LETTER LJ]
  611|      1|        '\u{1EFB}' => Some("ll"),  // ỻ [LATIN SMALL LETTER MIDDLE-WELSH LL]
  612|      1|        '\u{02AA}' => Some("ls"),  // ʪ [LATIN SMALL LETTER LS DIGRAPH]
  613|      1|        '\u{02AB}' => Some("lz"),  // ʫ [LATIN SMALL LETTER LZ DIGRAPH]
  614|      1|        '\u{019C}' => Some("M"),   // Ɯ [LATIN CAPITAL LETTER TURNED M]
  615|      1|        '\u{1D0D}' => Some("M"),   // ᴍ [LATIN LETTER SMALL CAPITAL M]
  616|      1|        '\u{1E3E}' => Some("M"),   // Ḿ [LATIN CAPITAL LETTER M WITH ACUTE]
  617|      1|        '\u{1E40}' => Some("M"),   // Ṁ [LATIN CAPITAL LETTER M WITH DOT ABOVE]
  618|      1|        '\u{1E42}' => Some("M"),   // Ṃ [LATIN CAPITAL LETTER M WITH DOT BELOW]
  619|      1|        '\u{24C2}' => Some("M"),   // Ⓜ [CIRCLED LATIN CAPITAL LETTER M]
  620|      1|        '\u{2C6E}' => Some("M"),   // Ɱ [LATIN CAPITAL LETTER M WITH HOOK]
  621|      1|        '\u{A7FD}' => Some("M"),   // ꟽ [LATIN EPIGRAPHIC LETTER INVERTED M]
  622|      1|        '\u{A7FF}' => Some("M"),   // ꟿ [LATIN EPIGRAPHIC LETTER ARCHAIC M]
  623|      1|        '\u{FF2D}' => Some("M"),   // Ｍ [FULLWIDTH LATIN CAPITAL LETTER M]
  624|      1|        '\u{026F}' => Some("m"),   // ɯ [LATIN SMALL LETTER TURNED M]
  625|      1|        '\u{0270}' => Some("m"),   /* ɰ [LATIN SMALL LETTER TURNED M WITH */
  626|       |        // LONG LEG]
  627|      1|        '\u{0271}' => Some("m"),   // ɱ [LATIN SMALL LETTER M WITH HOOK]
  628|      1|        '\u{1D6F}' => Some("m"),   // ᵯ [LATIN SMALL LETTER M WITH MIDDLE TILDE]
  629|      1|        '\u{1D86}' => Some("m"),   // ᶆ [LATIN SMALL LETTER M WITH PALATAL HOOK]
  630|      1|        '\u{1E3F}' => Some("m"),   // ḿ [LATIN SMALL LETTER M WITH ACUTE]
  631|      1|        '\u{1E41}' => Some("m"),   // ṁ [LATIN SMALL LETTER M WITH DOT ABOVE]
  632|      1|        '\u{1E43}' => Some("m"),   // ṃ [LATIN SMALL LETTER M WITH DOT BELOW]
  633|      1|        '\u{24DC}' => Some("m"),   // ⓜ [CIRCLED LATIN SMALL LETTER M]
  634|      1|        '\u{FF4D}' => Some("m"),   // ｍ [FULLWIDTH LATIN SMALL LETTER M]
  635|      1|        '\u{24A8}' => Some("(m)"), // ⒨ [PARENTHESIZED LATIN SMALL LETTER M]
  636|      1|        '\u{00D1}' => Some("N"),   // Ñ [LATIN CAPITAL LETTER N WITH TILDE]
  637|      1|        '\u{0143}' => Some("N"),   // Ń [LATIN CAPITAL LETTER N WITH ACUTE]
  638|      1|        '\u{0145}' => Some("N"),   // Ņ [LATIN CAPITAL LETTER N WITH CEDILLA]
  639|      1|        '\u{0147}' => Some("N"),   // Ň [LATIN CAPITAL LETTER N WITH CARON]
  640|      1|        '\u{014A}' => Some("N"),   /* Ŋ http://en.wikipedia.org/wiki/Eng_(letter) [LATIN CAPITAL LETTER ENG] */
  641|      1|        '\u{019D}' => Some("N"),   /* Ɲ [LATIN CAPITAL LETTER N WITH */
  642|       |        // ALIGN_LEFT HOOK]
  643|      1|        '\u{01F8}' => Some("N"), // Ǹ [LATIN CAPITAL LETTER N WITH GRAVE]
  644|      1|        '\u{0220}' => Some("N"), /* Ƞ [LATIN CAPITAL LETTER N WITH LONG */
  645|       |        // ALIGN_RIGHT LEG]
  646|      1|        '\u{0274}' => Some("N"), // ɴ [LATIN LETTER SMALL CAPITAL N]
  647|      1|        '\u{1D0E}' => Some("N"), // ᴎ [LATIN LETTER SMALL CAPITAL REVERSED N]
  648|      1|        '\u{1E44}' => Some("N"), // Ṅ [LATIN CAPITAL LETTER N WITH DOT ABOVE]
  649|      1|        '\u{1E46}' => Some("N"), // Ṇ [LATIN CAPITAL LETTER N WITH DOT BELOW]
  650|      1|        '\u{1E48}' => Some("N"), // Ṉ [LATIN CAPITAL LETTER N WITH LINE BELOW]
  651|      1|        '\u{1E4A}' => Some("N"), /* Ṋ [LATIN CAPITAL LETTER N WITH */
  652|       |        // CIRCUMFLEX BELOW]
  653|      1|        '\u{24C3}' => Some("N"), // Ⓝ [CIRCLED LATIN CAPITAL LETTER N]
  654|      1|        '\u{FF2E}' => Some("N"), // Ｎ [FULLWIDTH LATIN CAPITAL LETTER N]
  655|      1|        '\u{00F1}' => Some("n"), // ñ [LATIN SMALL LETTER N WITH TILDE]
  656|      1|        '\u{0144}' => Some("n"), // ń [LATIN SMALL LETTER N WITH ACUTE]
  657|      1|        '\u{0146}' => Some("n"), // ņ [LATIN SMALL LETTER N WITH CEDILLA]
  658|      1|        '\u{0148}' => Some("n"), // ň [LATIN SMALL LETTER N WITH CARON]
  659|      1|        '\u{0149}' => Some("n"), /* ŉ [LATIN SMALL LETTER N PRECEDED BY */
  660|       |        // APOSTROPHE]
  661|      1|        '\u{014B}' => Some("n"), /* ŋ http://en.wikipedia.org/wiki/Eng_(letter) [LATIN SMALL LETTER ENG] */
  662|      1|        '\u{019E}' => Some("n"), /* ƞ [LATIN SMALL LETTER N WITH LONG */
  663|       |        // ALIGN_RIGHT LEG]
  664|      1|        '\u{01F9}' => Some("n"), // ǹ [LATIN SMALL LETTER N WITH GRAVE]
  665|      1|        '\u{0235}' => Some("n"), // ȵ [LATIN SMALL LETTER N WITH CURL]
  666|      1|        '\u{0272}' => Some("n"), /* ɲ [LATIN SMALL LETTER N WITH ALIGN_LEFT */
  667|       |        // HOOK]
  668|      1|        '\u{0273}' => Some("n"), /* ɳ [LATIN SMALL LETTER N WITH RETROFLEX */
  669|       |        // HOOK]
  670|      1|        '\u{1D70}' => Some("n"), // ᵰ [LATIN SMALL LETTER N WITH MIDDLE TILDE]
  671|      1|        '\u{1D87}' => Some("n"), // ᶇ [LATIN SMALL LETTER N WITH PALATAL HOOK]
  672|      1|        '\u{1E45}' => Some("n"), // ṅ [LATIN SMALL LETTER N WITH DOT ABOVE]
  673|      1|        '\u{1E47}' => Some("n"), // ṇ [LATIN SMALL LETTER N WITH DOT BELOW]
  674|      1|        '\u{1E49}' => Some("n"), // ṉ [LATIN SMALL LETTER N WITH LINE BELOW]
  675|      1|        '\u{1E4B}' => Some("n"), /* ṋ [LATIN SMALL LETTER N WITH CIRCUMFLEX */
  676|       |        // BELOW]
  677|      1|        '\u{207F}' => Some("n"),  // ⁿ [SUPERSCRIPT LATIN SMALL LETTER N]
  678|      1|        '\u{24DD}' => Some("n"),  // ⓝ [CIRCLED LATIN SMALL LETTER N]
  679|      1|        '\u{FF4E}' => Some("n"),  // ｎ [FULLWIDTH LATIN SMALL LETTER N]
  680|      1|        '\u{01CA}' => Some("NJ"), // Ǌ [LATIN CAPITAL LETTER NJ]
  681|      1|        '\u{01CB}' => Some("Nj"), /* ǋ [LATIN CAPITAL LETTER N WITH SMALL */
  682|       |        // LETTER J]
  683|      1|        '\u{24A9}' => Some("(n)"), // ⒩ [PARENTHESIZED LATIN SMALL LETTER N]
  684|      1|        '\u{01CC}' => Some("nj"),  // ǌ [LATIN SMALL LETTER NJ]
  685|      1|        '\u{00D2}' => Some("O"),   // Ò [LATIN CAPITAL LETTER O WITH GRAVE]
  686|      1|        '\u{00D3}' => Some("O"),   // Ó [LATIN CAPITAL LETTER O WITH ACUTE]
  687|      1|        '\u{00D4}' => Some("O"),   /* Ô [LATIN CAPITAL LETTER O WITH */
  688|       |        // CIRCUMFLEX]
  689|      1|        '\u{00D5}' => Some("O"), // Õ [LATIN CAPITAL LETTER O WITH TILDE]
  690|      1|        '\u{00D6}' => Some("O"), // Ö [LATIN CAPITAL LETTER O WITH DIAERESIS]
  691|      1|        '\u{00D8}' => Some("O"), // Ø [LATIN CAPITAL LETTER O WITH STROKE]
  692|      1|        '\u{014C}' => Some("O"), // Ō [LATIN CAPITAL LETTER O WITH MACRON]
  693|      1|        '\u{014E}' => Some("O"), // Ŏ [LATIN CAPITAL LETTER O WITH BREVE]
  694|      1|        '\u{0150}' => Some("O"), /* Ő [LATIN CAPITAL LETTER O WITH DOUBLE */
  695|       |        // ACUTE]
  696|      1|        '\u{0186}' => Some("O"), // Ɔ [LATIN CAPITAL LETTER OPEN O]
  697|      1|        '\u{019F}' => Some("O"), /* Ɵ [LATIN CAPITAL LETTER O WITH MIDDLE */
  698|       |        // TILDE]
  699|      1|        '\u{01A0}' => Some("O"), // Ơ [LATIN CAPITAL LETTER O WITH HORN]
  700|      1|        '\u{01D1}' => Some("O"), // Ǒ [LATIN CAPITAL LETTER O WITH CARON]
  701|      1|        '\u{01EA}' => Some("O"), // Ǫ [LATIN CAPITAL LETTER O WITH OGONEK]
  702|      1|        '\u{01EC}' => Some("O"), /* Ǭ [LATIN CAPITAL LETTER O WITH OGONEK */
  703|       |        // AND MACRON]
  704|      1|        '\u{01FE}' => Some("O"), /* Ǿ [LATIN CAPITAL LETTER O WITH STROKE */
  705|       |        // AND ACUTE]
  706|      1|        '\u{020C}' => Some("O"), /* Ȍ [LATIN CAPITAL LETTER O WITH DOUBLE */
  707|       |        // GRAVE]
  708|      1|        '\u{020E}' => Some("O"), /* Ȏ [LATIN CAPITAL LETTER O WITH INVERTED */
  709|       |        // BREVE]
  710|      1|        '\u{022A}' => Some("O"), /* Ȫ [LATIN CAPITAL LETTER O WITH DIAERESIS */
  711|       |        // AND MACRON]
  712|      1|        '\u{022C}' => Some("O"), /* Ȭ [LATIN CAPITAL LETTER O WITH TILDE AND */
  713|       |        // MACRON]
  714|      1|        '\u{022E}' => Some("O"), // Ȯ [LATIN CAPITAL LETTER O WITH DOT ABOVE]
  715|      1|        '\u{0230}' => Some("O"), /* Ȱ [LATIN CAPITAL LETTER O WITH DOT ABOVE */
  716|       |        // AND MACRON]
  717|      1|        '\u{1D0F}' => Some("O"), // ᴏ [LATIN LETTER SMALL CAPITAL O]
  718|      1|        '\u{1D10}' => Some("O"), // ᴐ [LATIN LETTER SMALL CAPITAL OPEN O]
  719|      1|        '\u{1E4C}' => Some("O"), /* Ṍ [LATIN CAPITAL LETTER O WITH TILDE AND */
  720|       |        // ACUTE]
  721|      1|        '\u{1E4E}' => Some("O"), /* Ṏ [LATIN CAPITAL LETTER O WITH TILDE AND */
  722|       |        // DIAERESIS]
  723|      1|        '\u{1E50}' => Some("O"), /* Ṑ [LATIN CAPITAL LETTER O WITH MACRON */
  724|       |        // AND GRAVE]
  725|      1|        '\u{1E52}' => Some("O"), /* Ṓ [LATIN CAPITAL LETTER O WITH MACRON */
  726|       |        // AND ACUTE]
  727|      1|        '\u{1ECC}' => Some("O"), // Ọ [LATIN CAPITAL LETTER O WITH DOT BELOW]
  728|      1|        '\u{1ECE}' => Some("O"), // Ỏ [LATIN CAPITAL LETTER O WITH HOOK ABOVE]
  729|      1|        '\u{1ED0}' => Some("O"), /* Ố [LATIN CAPITAL LETTER O WITH */
  730|       |        // CIRCUMFLEX AND ACUTE]
  731|      1|        '\u{1ED2}' => Some("O"), /* Ồ [LATIN CAPITAL LETTER O WITH */
  732|       |        // CIRCUMFLEX AND GRAVE]
  733|      1|        '\u{1ED4}' => Some("O"), /* Ổ [LATIN CAPITAL LETTER O WITH */
  734|       |        // CIRCUMFLEX AND HOOK ABOVE]
  735|      1|        '\u{1ED6}' => Some("O"), /* Ỗ [LATIN CAPITAL LETTER O WITH */
  736|       |        // CIRCUMFLEX AND TILDE]
  737|      1|        '\u{1ED8}' => Some("O"), /* Ộ [LATIN CAPITAL LETTER O WITH */
  738|       |        // CIRCUMFLEX AND DOT BELOW]
  739|      1|        '\u{1EDA}' => Some("O"), /* Ớ [LATIN CAPITAL LETTER O WITH HORN AND */
  740|       |        // ACUTE]
  741|      1|        '\u{1EDC}' => Some("O"), /* Ờ [LATIN CAPITAL LETTER O WITH HORN AND */
  742|       |        // GRAVE]
  743|      1|        '\u{1EDE}' => Some("O"), /* Ở [LATIN CAPITAL LETTER O WITH HORN AND */
  744|       |        // HOOK ABOVE]
  745|      1|        '\u{1EE0}' => Some("O"), /* Ỡ [LATIN CAPITAL LETTER O WITH HORN AND */
  746|       |        // TILDE]
  747|      1|        '\u{1EE2}' => Some("O"), /* Ợ [LATIN CAPITAL LETTER O WITH HORN AND */
  748|       |        // DOT BELOW]
  749|      1|        '\u{24C4}' => Some("O"), // Ⓞ [CIRCLED LATIN CAPITAL LETTER O]
  750|      1|        '\u{A74A}' => Some("O"), /* Ꝋ [LATIN CAPITAL LETTER O WITH LONG */
  751|       |        // STROKE OVERLAY]
  752|      1|        '\u{A74C}' => Some("O"), // Ꝍ [LATIN CAPITAL LETTER O WITH LOOP]
  753|      1|        '\u{FF2F}' => Some("O"), // Ｏ [FULLWIDTH LATIN CAPITAL LETTER O]
  754|      1|        '\u{00F2}' => Some("o"), // ò [LATIN SMALL LETTER O WITH GRAVE]
  755|      1|        '\u{00F3}' => Some("o"), // ó [LATIN SMALL LETTER O WITH ACUTE]
  756|      1|        '\u{00F4}' => Some("o"), // ô [LATIN SMALL LETTER O WITH CIRCUMFLEX]
  757|      1|        '\u{00F5}' => Some("o"), // õ [LATIN SMALL LETTER O WITH TILDE]
  758|      1|        '\u{00F6}' => Some("o"), // ö [LATIN SMALL LETTER O WITH DIAERESIS]
  759|      1|        '\u{00F8}' => Some("o"), // ø [LATIN SMALL LETTER O WITH STROKE]
  760|      1|        '\u{014D}' => Some("o"), // ō [LATIN SMALL LETTER O WITH MACRON]
  761|      1|        '\u{014F}' => Some("o"), // ŏ [LATIN SMALL LETTER O WITH BREVE]
  762|      1|        '\u{0151}' => Some("o"), // ő [LATIN SMALL LETTER O WITH DOUBLE ACUTE]
  763|      1|        '\u{01A1}' => Some("o"), // ơ [LATIN SMALL LETTER O WITH HORN]
  764|      1|        '\u{01D2}' => Some("o"), // ǒ [LATIN SMALL LETTER O WITH CARON]
  765|      1|        '\u{01EB}' => Some("o"), // ǫ [LATIN SMALL LETTER O WITH OGONEK]
  766|      1|        '\u{01ED}' => Some("o"), /* ǭ [LATIN SMALL LETTER O WITH OGONEK AND */
  767|       |        // MACRON]
  768|      1|        '\u{01FF}' => Some("o"), /* ǿ [LATIN SMALL LETTER O WITH STROKE AND */
  769|       |        // ACUTE]
  770|      1|        '\u{020D}' => Some("o"), // ȍ [LATIN SMALL LETTER O WITH DOUBLE GRAVE]
  771|      1|        '\u{020F}' => Some("o"), /* ȏ [LATIN SMALL LETTER O WITH INVERTED */
  772|       |        // BREVE]
  773|      1|        '\u{022B}' => Some("o"), /* ȫ [LATIN SMALL LETTER O WITH DIAERESIS */
  774|       |        // AND MACRON]
  775|      1|        '\u{022D}' => Some("o"), /* ȭ [LATIN SMALL LETTER O WITH TILDE AND */
  776|       |        // MACRON]
  777|      1|        '\u{022F}' => Some("o"), // ȯ [LATIN SMALL LETTER O WITH DOT ABOVE]
  778|      1|        '\u{0231}' => Some("o"), /* ȱ [LATIN SMALL LETTER O WITH DOT ABOVE */
  779|       |        // AND MACRON]
  780|      1|        '\u{0254}' => Some("o"), // ɔ [LATIN SMALL LETTER OPEN O]
  781|      1|        '\u{0275}' => Some("o"), // ɵ [LATIN SMALL LETTER BARRED O]
  782|      1|        '\u{1D16}' => Some("o"), // ᴖ [LATIN SMALL LETTER TOP HALF O]
  783|      1|        '\u{1D17}' => Some("o"), // ᴗ [LATIN SMALL LETTER BOTTOM HALF O]
  784|      1|        '\u{1D97}' => Some("o"), /* ᶗ [LATIN SMALL LETTER OPEN O WITH */
  785|       |        // RETROFLEX HOOK]
  786|      1|        '\u{1E4D}' => Some("o"), /* ṍ [LATIN SMALL LETTER O WITH TILDE AND */
  787|       |        // ACUTE]
  788|      1|        '\u{1E4F}' => Some("o"), /* ṏ [LATIN SMALL LETTER O WITH TILDE AND */
  789|       |        // DIAERESIS]
  790|      1|        '\u{1E51}' => Some("o"), /* ṑ [LATIN SMALL LETTER O WITH MACRON AND */
  791|       |        // GRAVE]
  792|      1|        '\u{1E53}' => Some("o"), /* ṓ [LATIN SMALL LETTER O WITH MACRON AND */
  793|       |        // ACUTE]
  794|      1|        '\u{1ECD}' => Some("o"), // ọ [LATIN SMALL LETTER O WITH DOT BELOW]
  795|      1|        '\u{1ECF}' => Some("o"), // ỏ [LATIN SMALL LETTER O WITH HOOK ABOVE]
  796|      1|        '\u{1ED1}' => Some("o"), /* ố [LATIN SMALL LETTER O WITH CIRCUMFLEX */
  797|       |        // AND ACUTE]
  798|      1|        '\u{1ED3}' => Some("o"), /* ồ [LATIN SMALL LETTER O WITH CIRCUMFLEX */
  799|       |        // AND GRAVE]
  800|      1|        '\u{1ED5}' => Some("o"), /* ổ [LATIN SMALL LETTER O WITH CIRCUMFLEX */
  801|       |        // AND HOOK ABOVE]
  802|      1|        '\u{1ED7}' => Some("o"), /* ỗ [LATIN SMALL LETTER O WITH CIRCUMFLEX */
  803|       |        // AND TILDE]
  804|      1|        '\u{1ED9}' => Some("o"), /* ộ [LATIN SMALL LETTER O WITH CIRCUMFLEX */
  805|       |        // AND DOT BELOW]
  806|      1|        '\u{1EDB}' => Some("o"), /* ớ [LATIN SMALL LETTER O WITH HORN AND */
  807|       |        // ACUTE]
  808|      1|        '\u{1EDD}' => Some("o"), /* ờ [LATIN SMALL LETTER O WITH HORN AND */
  809|       |        // GRAVE]
  810|      1|        '\u{1EDF}' => Some("o"), /* ở [LATIN SMALL LETTER O WITH HORN AND */
  811|       |        // HOOK ABOVE]
  812|      1|        '\u{1EE1}' => Some("o"), /* ỡ [LATIN SMALL LETTER O WITH HORN AND */
  813|       |        // TILDE]
  814|      1|        '\u{1EE3}' => Some("o"), /* ợ [LATIN SMALL LETTER O WITH HORN AND */
  815|       |        // DOT BELOW]
  816|      1|        '\u{2092}' => Some("o"), // ₒ [LATIN SUBSCRIPT SMALL LETTER O]
  817|      1|        '\u{24DE}' => Some("o"), // ⓞ [CIRCLED LATIN SMALL LETTER O]
  818|      1|        '\u{2C7A}' => Some("o"), /* ⱺ [LATIN SMALL LETTER O WITH LOW RING */
  819|       |        // INSIDE]
  820|      1|        '\u{A74B}' => Some("o"), /* ꝋ [LATIN SMALL LETTER O WITH LONG STROKE */
  821|       |        // OVERLAY]
  822|      1|        '\u{A74D}' => Some("o"),   // ꝍ [LATIN SMALL LETTER O WITH LOOP]
  823|      1|        '\u{FF4F}' => Some("o"),   // ｏ [FULLWIDTH LATIN SMALL LETTER O]
  824|      1|        '\u{0152}' => Some("OE"),  // Œ [LATIN CAPITAL LIGATURE OE]
  825|      1|        '\u{0276}' => Some("OE"),  // ɶ [LATIN LETTER SMALL CAPITAL OE]
  826|      1|        '\u{A74E}' => Some("OO"),  // Ꝏ [LATIN CAPITAL LETTER OO]
  827|      1|        '\u{0222}' => Some("OU"),  /* Ȣ http://en.wikipedia.org/wiki/OU [LATIN CAPITAL LETTER OU] */
  828|      1|        '\u{1D15}' => Some("OU"),  // ᴕ [LATIN LETTER SMALL CAPITAL OU]
  829|      1|        '\u{24AA}' => Some("(o)"), // ⒪ [PARENTHESIZED LATIN SMALL LETTER O]
  830|      1|        '\u{0153}' => Some("oe"),  // œ [LATIN SMALL LIGATURE OE]
  831|      1|        '\u{1D14}' => Some("oe"),  // ᴔ [LATIN SMALL LETTER TURNED OE]
  832|      1|        '\u{A74F}' => Some("oo"),  // ꝏ [LATIN SMALL LETTER OO]
  833|      1|        '\u{0223}' => Some("ou"),  /* ȣ http://en.wikipedia.org/wiki/OU [LATIN SMALL LETTER OU] */
  834|      1|        '\u{01A4}' => Some("P"),   // Ƥ [LATIN CAPITAL LETTER P WITH HOOK]
  835|      1|        '\u{1D18}' => Some("P"),   // ᴘ [LATIN LETTER SMALL CAPITAL P]
  836|      1|        '\u{1E54}' => Some("P"),   // Ṕ [LATIN CAPITAL LETTER P WITH ACUTE]
  837|      1|        '\u{1E56}' => Some("P"),   // Ṗ [LATIN CAPITAL LETTER P WITH DOT ABOVE]
  838|      1|        '\u{24C5}' => Some("P"),   // Ⓟ [CIRCLED LATIN CAPITAL LETTER P]
  839|      1|        '\u{2C63}' => Some("P"),   // Ᵽ [LATIN CAPITAL LETTER P WITH STROKE]
  840|      1|        '\u{A750}' => Some("P"),   /* Ꝑ [LATIN CAPITAL LETTER P WITH STROKE */
  841|       |        // THROUGH DESCENDER]
  842|      1|        '\u{A752}' => Some("P"), // Ꝓ [LATIN CAPITAL LETTER P WITH FLOURISH]
  843|      1|        '\u{A754}' => Some("P"), /* Ꝕ [LATIN CAPITAL LETTER P WITH SQUIRREL */
  844|       |        // TAIL]
  845|      1|        '\u{FF30}' => Some("P"), // Ｐ [FULLWIDTH LATIN CAPITAL LETTER P]
  846|      1|        '\u{01A5}' => Some("p"), // ƥ [LATIN SMALL LETTER P WITH HOOK]
  847|      1|        '\u{1D71}' => Some("p"), // ᵱ [LATIN SMALL LETTER P WITH MIDDLE TILDE]
  848|      1|        '\u{1D7D}' => Some("p"), // ᵽ [LATIN SMALL LETTER P WITH STROKE]
  849|      1|        '\u{1D88}' => Some("p"), // ᶈ [LATIN SMALL LETTER P WITH PALATAL HOOK]
  850|      1|        '\u{1E55}' => Some("p"), // ṕ [LATIN SMALL LETTER P WITH ACUTE]
  851|      1|        '\u{1E57}' => Some("p"), // ṗ [LATIN SMALL LETTER P WITH DOT ABOVE]
  852|      1|        '\u{24DF}' => Some("p"), // ⓟ [CIRCLED LATIN SMALL LETTER P]
  853|      1|        '\u{A751}' => Some("p"), /* ꝑ [LATIN SMALL LETTER P WITH STROKE */
  854|       |        // THROUGH DESCENDER]
  855|      1|        '\u{A753}' => Some("p"),   // ꝓ [LATIN SMALL LETTER P WITH FLOURISH]
  856|      1|        '\u{A755}' => Some("p"),   // ꝕ [LATIN SMALL LETTER P WITH SQUIRREL TAIL]
  857|      1|        '\u{A7FC}' => Some("p"),   // ꟼ [LATIN EPIGRAPHIC LETTER REVERSED P]
  858|      1|        '\u{FF50}' => Some("p"),   // ｐ [FULLWIDTH LATIN SMALL LETTER P]
  859|      1|        '\u{24AB}' => Some("(p)"), // ⒫ [PARENTHESIZED LATIN SMALL LETTER P]
  860|      1|        '\u{024A}' => Some("Q"),   /* Ɋ [LATIN CAPITAL LETTER SMALL Q WITH */
  861|       |        // HOOK TAIL]
  862|      1|        '\u{24C6}' => Some("Q"), // Ⓠ [CIRCLED LATIN CAPITAL LETTER Q]
  863|      1|        '\u{A756}' => Some("Q"), /* Ꝗ [LATIN CAPITAL LETTER Q WITH STROKE */
  864|       |        // THROUGH DESCENDER]
  865|      1|        '\u{A758}' => Some("Q"), /* Ꝙ [LATIN CAPITAL LETTER Q WITH DIAGONAL */
  866|       |        // STROKE]
  867|      1|        '\u{FF31}' => Some("Q"), // Ｑ [FULLWIDTH LATIN CAPITAL LETTER Q]
  868|      1|        '\u{0138}' => Some("q"), /* ĸ http://en.wikipedia.org/wiki/Kra_(letter) [LATIN SMALL LETTER KRA] */
  869|      1|        '\u{024B}' => Some("q"), // ɋ [LATIN SMALL LETTER Q WITH HOOK TAIL]
  870|      1|        '\u{02A0}' => Some("q"), // ʠ [LATIN SMALL LETTER Q WITH HOOK]
  871|      1|        '\u{24E0}' => Some("q"), // ⓠ [CIRCLED LATIN SMALL LETTER Q]
  872|      1|        '\u{A757}' => Some("q"), /* ꝗ [LATIN SMALL LETTER Q WITH STROKE */
  873|       |        // THROUGH DESCENDER]
  874|      1|        '\u{A759}' => Some("q"), /* ꝙ [LATIN SMALL LETTER Q WITH DIAGONAL */
  875|       |        // STROKE]
  876|      1|        '\u{FF51}' => Some("q"),   // ｑ [FULLWIDTH LATIN SMALL LETTER Q]
  877|      1|        '\u{24AC}' => Some("(q)"), // ⒬ [PARENTHESIZED LATIN SMALL LETTER Q]
  878|      1|        '\u{0239}' => Some("qp"),  // ȹ [LATIN SMALL LETTER QP DIGRAPH]
  879|      1|        '\u{0154}' => Some("R"),   // Ŕ [LATIN CAPITAL LETTER R WITH ACUTE]
  880|      1|        '\u{0156}' => Some("R"),   // Ŗ [LATIN CAPITAL LETTER R WITH CEDILLA]
  881|      1|        '\u{0158}' => Some("R"),   // Ř [LATIN CAPITAL LETTER R WITH CARON]
  882|      1|        '\u{0210}' => Some("R"),   /* Ȓ [LATIN CAPITAL LETTER R WITH DOUBLE */
  883|       |        // GRAVE]
  884|      1|        '\u{0212}' => Some("R"), /* Ȓ [LATIN CAPITAL LETTER R WITH INVERTED */
  885|       |        // BREVE]
  886|      1|        '\u{024C}' => Some("R"), // Ɍ [LATIN CAPITAL LETTER R WITH STROKE]
  887|      1|        '\u{0280}' => Some("R"), // ʀ [LATIN LETTER SMALL CAPITAL R]
  888|      1|        '\u{0281}' => Some("R"), // ʁ [LATIN LETTER SMALL CAPITAL INVERTED R]
  889|      1|        '\u{1D19}' => Some("R"), // ᴙ [LATIN LETTER SMALL CAPITAL REVERSED R]
  890|      1|        '\u{1D1A}' => Some("R"), // ᴚ [LATIN LETTER SMALL CAPITAL TURNED R]
  891|      1|        '\u{1E58}' => Some("R"), // Ṙ [LATIN CAPITAL LETTER R WITH DOT ABOVE]
  892|      1|        '\u{1E5A}' => Some("R"), // Ṛ [LATIN CAPITAL LETTER R WITH DOT BELOW]
  893|      1|        '\u{1E5C}' => Some("R"), /* Ṝ [LATIN CAPITAL LETTER R WITH DOT BELOW */
  894|       |        // AND MACRON]
  895|      1|        '\u{1E5E}' => Some("R"), // Ṟ [LATIN CAPITAL LETTER R WITH LINE BELOW]
  896|      1|        '\u{24C7}' => Some("R"), // Ⓡ [CIRCLED LATIN CAPITAL LETTER R]
  897|      1|        '\u{2C64}' => Some("R"), // Ɽ [LATIN CAPITAL LETTER R WITH TAIL]
  898|      1|        '\u{A75A}' => Some("R"), // Ꝛ [LATIN CAPITAL LETTER R ROTUNDA]
  899|      1|        '\u{A782}' => Some("R"), // Ꞃ [LATIN CAPITAL LETTER INSULAR R]
  900|      1|        '\u{FF32}' => Some("R"), // Ｒ [FULLWIDTH LATIN CAPITAL LETTER R]
  901|      1|        '\u{0155}' => Some("r"), // ŕ [LATIN SMALL LETTER R WITH ACUTE]
  902|      1|        '\u{0157}' => Some("r"), // ŗ [LATIN SMALL LETTER R WITH CEDILLA]
  903|      1|        '\u{0159}' => Some("r"), // ř [LATIN SMALL LETTER R WITH CARON]
  904|      1|        '\u{0211}' => Some("r"), // ȑ [LATIN SMALL LETTER R WITH DOUBLE GRAVE]
  905|      1|        '\u{0213}' => Some("r"), /* ȓ [LATIN SMALL LETTER R WITH INVERTED */
  906|       |        // BREVE]
  907|      1|        '\u{024D}' => Some("r"), // ɍ [LATIN SMALL LETTER R WITH STROKE]
  908|      1|        '\u{027C}' => Some("r"), // ɼ [LATIN SMALL LETTER R WITH LONG LEG]
  909|      1|        '\u{027D}' => Some("r"), // ɽ [LATIN SMALL LETTER R WITH TAIL]
  910|      1|        '\u{027E}' => Some("r"), // ɾ [LATIN SMALL LETTER R WITH FISHHOOK]
  911|      1|        '\u{027F}' => Some("r"), /* ɿ [LATIN SMALL LETTER REVERSED R WITH */
  912|       |        // FISHHOOK]
  913|      1|        '\u{1D63}' => Some("r"), // ᵣ [LATIN SUBSCRIPT SMALL LETTER R]
  914|      1|        '\u{1D72}' => Some("r"), // ᵲ [LATIN SMALL LETTER R WITH MIDDLE TILDE]
  915|      1|        '\u{1D73}' => Some("r"), /* ᵳ [LATIN SMALL LETTER R WITH FISHHOOK */
  916|       |        // AND MIDDLE TILDE]
  917|      1|        '\u{1D89}' => Some("r"), // ᶉ [LATIN SMALL LETTER R WITH PALATAL HOOK]
  918|      1|        '\u{1E59}' => Some("r"), // ṙ [LATIN SMALL LETTER R WITH DOT ABOVE]
  919|      1|        '\u{1E5B}' => Some("r"), // ṛ [LATIN SMALL LETTER R WITH DOT BELOW]
  920|      1|        '\u{1E5D}' => Some("r"), /* ṝ [LATIN SMALL LETTER R WITH DOT BELOW */
  921|       |        // AND MACRON]
  922|      1|        '\u{1E5F}' => Some("r"),   // ṟ [LATIN SMALL LETTER R WITH LINE BELOW]
  923|      1|        '\u{24E1}' => Some("r"),   // ⓡ [CIRCLED LATIN SMALL LETTER R]
  924|      1|        '\u{A75B}' => Some("r"),   // ꝛ [LATIN SMALL LETTER R ROTUNDA]
  925|      1|        '\u{A783}' => Some("r"),   // ꞃ [LATIN SMALL LETTER INSULAR R]
  926|      1|        '\u{FF52}' => Some("r"),   // ｒ [FULLWIDTH LATIN SMALL LETTER R]
  927|      1|        '\u{24AD}' => Some("(r)"), // ⒭ [PARENTHESIZED LATIN SMALL LETTER R]
  928|      1|        '\u{015A}' => Some("S"),   // Ś [LATIN CAPITAL LETTER S WITH ACUTE]
  929|      1|        '\u{015C}' => Some("S"),   // Ŝ [LATIN CAPITAL LETTER S WITH CIRCUMFLEX]
  930|      1|        '\u{015E}' => Some("S"),   // Ş [LATIN CAPITAL LETTER S WITH CEDILLA]
  931|      1|        '\u{0160}' => Some("S"),   // Š [LATIN CAPITAL LETTER S WITH CARON]
  932|      1|        '\u{0218}' => Some("S"),   // Ș [LATIN CAPITAL LETTER S WITH COMMA BELOW]
  933|      1|        '\u{1E60}' => Some("S"),   // Ṡ [LATIN CAPITAL LETTER S WITH DOT ABOVE]
  934|      1|        '\u{1E62}' => Some("S"),   // Ṣ [LATIN CAPITAL LETTER S WITH DOT BELOW]
  935|      1|        '\u{1E64}' => Some("S"),   /* Ṥ [LATIN CAPITAL LETTER S WITH ACUTE AND */
  936|       |        // DOT ABOVE]
  937|      1|        '\u{1E66}' => Some("S"), /* Ṧ [LATIN CAPITAL LETTER S WITH CARON AND */
  938|       |        // DOT ABOVE]
  939|      1|        '\u{1E68}' => Some("S"), /* Ṩ [LATIN CAPITAL LETTER S WITH DOT BELOW */
  940|       |        // AND DOT ABOVE]
  941|      1|        '\u{24C8}' => Some("S"), // Ⓢ [CIRCLED LATIN CAPITAL LETTER S]
  942|      1|        '\u{A731}' => Some("S"), // ꜱ [LATIN LETTER SMALL CAPITAL S]
  943|      1|        '\u{A785}' => Some("S"), // ꞅ [LATIN SMALL LETTER INSULAR S]
  944|      1|        '\u{FF33}' => Some("S"), // Ｓ [FULLWIDTH LATIN CAPITAL LETTER S]
  945|      1|        '\u{015B}' => Some("s"), // ś [LATIN SMALL LETTER S WITH ACUTE]
  946|      1|        '\u{015D}' => Some("s"), // ŝ [LATIN SMALL LETTER S WITH CIRCUMFLEX]
  947|      1|        '\u{015F}' => Some("s"), // ş [LATIN SMALL LETTER S WITH CEDILLA]
  948|      1|        '\u{0161}' => Some("s"), // š [LATIN SMALL LETTER S WITH CARON]
  949|      1|        '\u{017F}' => Some("s"), /* ſ http://en.wikipedia.org/wiki/Long_S [LATIN SMALL */
  950|      1|        '\u{0219}' => Some("s"), // ș [LATIN SMALL LETTER S WITH COMMA BELOW]
  951|      1|        '\u{023F}' => Some("s"), // ȿ [LATIN SMALL LETTER S WITH SWASH TAIL]
  952|      1|        '\u{0282}' => Some("s"), // ʂ [LATIN SMALL LETTER S WITH HOOK]
  953|      1|        '\u{1D74}' => Some("s"), // ᵴ [LATIN SMALL LETTER S WITH MIDDLE TILDE]
  954|      1|        '\u{1D8A}' => Some("s"), // ᶊ [LATIN SMALL LETTER S WITH PALATAL HOOK]
  955|      1|        '\u{1E61}' => Some("s"), // ṡ [LATIN SMALL LETTER S WITH DOT ABOVE]
  956|      1|        '\u{1E63}' => Some("s"), // ṣ [LATIN SMALL LETTER S WITH DOT BELOW]
  957|      1|        '\u{1E65}' => Some("s"), /* ṥ [LATIN SMALL LETTER S WITH ACUTE AND */
  958|       |        // DOT ABOVE]
  959|      1|        '\u{1E67}' => Some("s"), /* ṧ [LATIN SMALL LETTER S WITH CARON AND */
  960|       |        // DOT ABOVE]
  961|      1|        '\u{1E69}' => Some("s"), /* ṩ [LATIN SMALL LETTER S WITH DOT BELOW */
  962|       |        // AND DOT ABOVE]
  963|      1|        '\u{1E9C}' => Some("s"), /* ẜ [LATIN SMALL LETTER LONG S WITH */
  964|       |        // DIAGONAL STROKE]
  965|      1|        '\u{1E9D}' => Some("s"), /* ẝ [LATIN SMALL LETTER LONG S WITH HIGH */
  966|       |        // STROKE]
  967|      1|        '\u{24E2}' => Some("s"),   // ⓢ [CIRCLED LATIN SMALL LETTER S]
  968|      1|        '\u{A784}' => Some("s"),   // Ꞅ [LATIN CAPITAL LETTER INSULAR S]
  969|      1|        '\u{FF53}' => Some("s"),   // ｓ [FULLWIDTH LATIN SMALL LETTER S]
  970|      1|        '\u{1E9E}' => Some("SS"),  // ẞ [LATIN CAPITAL LETTER SHARP S]
  971|      1|        '\u{24AE}' => Some("(s)"), // ⒮ [PARENTHESIZED LATIN SMALL LETTER S]
  972|      1|        '\u{00DF}' => Some("ss"),  // ß [LATIN SMALL LETTER SHARP S]
  973|      1|        '\u{FB06}' => Some("st"),  // ﬆ [LATIN SMALL LIGATURE ST]
  974|      1|        '\u{0162}' => Some("T"),   // Ţ [LATIN CAPITAL LETTER T WITH CEDILLA]
  975|      1|        '\u{0164}' => Some("T"),   // Ť [LATIN CAPITAL LETTER T WITH CARON]
  976|      1|        '\u{0166}' => Some("T"),   // Ŧ [LATIN CAPITAL LETTER T WITH STROKE]
  977|      1|        '\u{01AC}' => Some("T"),   // Ƭ [LATIN CAPITAL LETTER T WITH HOOK]
  978|      1|        '\u{01AE}' => Some("T"),   /* Ʈ [LATIN CAPITAL LETTER T WITH RETROFLEX */
  979|       |        // HOOK]
  980|      1|        '\u{021A}' => Some("T"), // Ț [LATIN CAPITAL LETTER T WITH COMMA BELOW]
  981|      1|        '\u{023E}' => Some("T"), /* Ⱦ [LATIN CAPITAL LETTER T WITH DIAGONAL */
  982|       |        // STROKE]
  983|      1|        '\u{1D1B}' => Some("T"), // ᴛ [LATIN LETTER SMALL CAPITAL T]
  984|      1|        '\u{1E6A}' => Some("T"), // Ṫ [LATIN CAPITAL LETTER T WITH DOT ABOVE]
  985|      1|        '\u{1E6C}' => Some("T"), // Ṭ [LATIN CAPITAL LETTER T WITH DOT BELOW]
  986|      1|        '\u{1E6E}' => Some("T"), // Ṯ [LATIN CAPITAL LETTER T WITH LINE BELOW]
  987|      1|        '\u{1E70}' => Some("T"), /* Ṱ [LATIN CAPITAL LETTER T WITH */
  988|       |        // CIRCUMFLEX BELOW]
  989|      1|        '\u{24C9}' => Some("T"), // Ⓣ [CIRCLED LATIN CAPITAL LETTER T]
  990|      1|        '\u{A786}' => Some("T"), // Ꞇ [LATIN CAPITAL LETTER INSULAR T]
  991|      1|        '\u{FF34}' => Some("T"), // Ｔ [FULLWIDTH LATIN CAPITAL LETTER T]
  992|      1|        '\u{0163}' => Some("t"), // ţ [LATIN SMALL LETTER T WITH CEDILLA]
  993|      1|        '\u{0165}' => Some("t"), // ť [LATIN SMALL LETTER T WITH CARON]
  994|      1|        '\u{0167}' => Some("t"), // ŧ [LATIN SMALL LETTER T WITH STROKE]
  995|      1|        '\u{01AB}' => Some("t"), // ƫ [LATIN SMALL LETTER T WITH PALATAL HOOK]
  996|      1|        '\u{01AD}' => Some("t"), // ƭ [LATIN SMALL LETTER T WITH HOOK]
  997|      1|        '\u{021B}' => Some("t"), // ț [LATIN SMALL LETTER T WITH COMMA BELOW]
  998|      1|        '\u{0236}' => Some("t"), // ȶ [LATIN SMALL LETTER T WITH CURL]
  999|      1|        '\u{0287}' => Some("t"), // ʇ [LATIN SMALL LETTER TURNED T]
 1000|      1|        '\u{0288}' => Some("t"), /* ʈ [LATIN SMALL LETTER T WITH RETROFLEX */
 1001|       |        // HOOK]
 1002|      1|        '\u{1D75}' => Some("t"), // ᵵ [LATIN SMALL LETTER T WITH MIDDLE TILDE]
 1003|      1|        '\u{1E6B}' => Some("t"), // ṫ [LATIN SMALL LETTER T WITH DOT ABOVE]
 1004|      1|        '\u{1E6D}' => Some("t"), // ṭ [LATIN SMALL LETTER T WITH DOT BELOW]
 1005|      1|        '\u{1E6F}' => Some("t"), // ṯ [LATIN SMALL LETTER T WITH LINE BELOW]
 1006|      1|        '\u{1E71}' => Some("t"), /* ṱ [LATIN SMALL LETTER T WITH CIRCUMFLEX */
 1007|       |        // BELOW]
 1008|      1|        '\u{1E97}' => Some("t"), // ẗ [LATIN SMALL LETTER T WITH DIAERESIS]
 1009|      1|        '\u{24E3}' => Some("t"), // ⓣ [CIRCLED LATIN SMALL LETTER T]
 1010|      1|        '\u{2C66}' => Some("t"), /* ⱦ [LATIN SMALL LETTER T WITH DIAGONAL */
 1011|       |        // STROKE]
 1012|      1|        '\u{FF54}' => Some("t"),  // ｔ [FULLWIDTH LATIN SMALL LETTER T]
 1013|      1|        '\u{00DE}' => Some("TH"), // Þ [LATIN CAPITAL LETTER THORN]
 1014|      1|        '\u{A766}' => Some("TH"), /* Ꝧ [LATIN CAPITAL LETTER THORN WITH */
 1015|       |        // STROKE THROUGH DESCENDER]
 1016|      1|        '\u{A728}' => Some("TZ"),  // Ꜩ [LATIN CAPITAL LETTER TZ]
 1017|      1|        '\u{24AF}' => Some("(t)"), // ⒯ [PARENTHESIZED LATIN SMALL LETTER T]
 1018|      1|        '\u{02A8}' => Some("tc"),  /* ʨ [LATIN SMALL LETTER TC DIGRAPH WITH */
 1019|       |        // CURL]
 1020|      1|        '\u{00FE}' => Some("th"), // þ [LATIN SMALL LETTER THORN]
 1021|      1|        '\u{1D7A}' => Some("th"), /* ᵺ [LATIN SMALL LETTER TH WITH */
 1022|       |        // STRIKETHROUGH]
 1023|      1|        '\u{A767}' => Some("th"), /* ꝧ [LATIN SMALL LETTER THORN WITH STROKE */
 1024|       |        // THROUGH DESCENDER]
 1025|      1|        '\u{02A6}' => Some("ts"), // ʦ [LATIN SMALL LETTER TS DIGRAPH]
 1026|      1|        '\u{A729}' => Some("tz"), // ꜩ [LATIN SMALL LETTER TZ]
 1027|      1|        '\u{00D9}' => Some("U"),  // Ù [LATIN CAPITAL LETTER U WITH GRAVE]
 1028|      1|        '\u{00DA}' => Some("U"),  // Ú [LATIN CAPITAL LETTER U WITH ACUTE]
 1029|      1|        '\u{00DB}' => Some("U"),  // Û [LATIN CAPITAL LETTER U WITH CIRCUMFLEX]
 1030|      1|        '\u{00DC}' => Some("U"),  // Ü [LATIN CAPITAL LETTER U WITH DIAERESIS]
 1031|      1|        '\u{0168}' => Some("U"),  // Ũ [LATIN CAPITAL LETTER U WITH TILDE]
 1032|      1|        '\u{016A}' => Some("U"),  // Ū [LATIN CAPITAL LETTER U WITH MACRON]
 1033|      1|        '\u{016C}' => Some("U"),  // Ŭ [LATIN CAPITAL LETTER U WITH BREVE]
 1034|      1|        '\u{016E}' => Some("U"),  // Ů [LATIN CAPITAL LETTER U WITH RING ABOVE]
 1035|      1|        '\u{0170}' => Some("U"),  /* Ű [LATIN CAPITAL LETTER U WITH DOUBLE */
 1036|       |        // ACUTE]
 1037|      1|        '\u{0172}' => Some("U"), // Ų [LATIN CAPITAL LETTER U WITH OGONEK]
 1038|      1|        '\u{01AF}' => Some("U"), // Ư [LATIN CAPITAL LETTER U WITH HORN]
 1039|      1|        '\u{01D3}' => Some("U"), // Ǔ [LATIN CAPITAL LETTER U WITH CARON]
 1040|      1|        '\u{01D5}' => Some("U"), /* Ǖ [LATIN CAPITAL LETTER U WITH DIAERESIS */
 1041|       |        // AND MACRON]
 1042|      1|        '\u{01D7}' => Some("U"), /* Ǘ [LATIN CAPITAL LETTER U WITH DIAERESIS */
 1043|       |        // AND ACUTE]
 1044|      1|        '\u{01D9}' => Some("U"), /* Ǚ [LATIN CAPITAL LETTER U WITH DIAERESIS */
 1045|       |        // AND CARON]
 1046|      1|        '\u{01DB}' => Some("U"), /* Ǜ [LATIN CAPITAL LETTER U WITH DIAERESIS */
 1047|       |        // AND GRAVE]
 1048|      1|        '\u{0214}' => Some("U"), /* Ȕ [LATIN CAPITAL LETTER U WITH DOUBLE */
 1049|       |        // GRAVE]
 1050|      1|        '\u{0216}' => Some("U"), /* Ȗ [LATIN CAPITAL LETTER U WITH INVERTED */
 1051|       |        // BREVE]
 1052|      1|        '\u{0244}' => Some("U"), // Ʉ [LATIN CAPITAL LETTER U BAR]
 1053|      1|        '\u{1D1C}' => Some("U"), // ᴜ [LATIN LETTER SMALL CAPITAL U]
 1054|      1|        '\u{1D7E}' => Some("U"), /* ᵾ [LATIN SMALL CAPITAL LETTER U WITH */
 1055|       |        // STROKE]
 1056|      1|        '\u{1E72}' => Some("U"), /* Ṳ [LATIN CAPITAL LETTER U WITH DIAERESIS */
 1057|       |        // BELOW]
 1058|      1|        '\u{1E74}' => Some("U"), // Ṵ [LATIN CAPITAL LETTER U WITH TILDE BELOW]
 1059|      1|        '\u{1E76}' => Some("U"), /* Ṷ [LATIN CAPITAL LETTER U WITH */
 1060|       |        // CIRCUMFLEX BELOW]
 1061|      1|        '\u{1E78}' => Some("U"), /* Ṹ [LATIN CAPITAL LETTER U WITH TILDE AND */
 1062|       |        // ACUTE]
 1063|      1|        '\u{1E7A}' => Some("U"), /* Ṻ [LATIN CAPITAL LETTER U WITH MACRON */
 1064|       |        // AND DIAERESIS]
 1065|      1|        '\u{1EE4}' => Some("U"), // Ụ [LATIN CAPITAL LETTER U WITH DOT BELOW]
 1066|      1|        '\u{1EE6}' => Some("U"), // Ủ [LATIN CAPITAL LETTER U WITH HOOK ABOVE]
 1067|      1|        '\u{1EE8}' => Some("U"), /* Ứ [LATIN CAPITAL LETTER U WITH HORN AND */
 1068|       |        // ACUTE]
 1069|      1|        '\u{1EEA}' => Some("U"), /* Ừ [LATIN CAPITAL LETTER U WITH HORN AND */
 1070|       |        // GRAVE]
 1071|      1|        '\u{1EEC}' => Some("U"), /* Ử [LATIN CAPITAL LETTER U WITH HORN AND */
 1072|       |        // HOOK ABOVE]
 1073|      1|        '\u{1EEE}' => Some("U"), /* Ữ [LATIN CAPITAL LETTER U WITH HORN AND */
 1074|       |        // TILDE]
 1075|      1|        '\u{1EF0}' => Some("U"), /* Ự [LATIN CAPITAL LETTER U WITH HORN AND */
 1076|       |        // DOT BELOW]
 1077|      1|        '\u{24CA}' => Some("U"), // Ⓤ [CIRCLED LATIN CAPITAL LETTER U]
 1078|      1|        '\u{FF35}' => Some("U"), // Ｕ [FULLWIDTH LATIN CAPITAL LETTER U]
 1079|      1|        '\u{00F9}' => Some("u"), // ù [LATIN SMALL LETTER U WITH GRAVE]
 1080|      1|        '\u{00FA}' => Some("u"), // ú [LATIN SMALL LETTER U WITH ACUTE]
 1081|      1|        '\u{00FB}' => Some("u"), // û [LATIN SMALL LETTER U WITH CIRCUMFLEX]
 1082|      1|        '\u{00FC}' => Some("u"), // ü [LATIN SMALL LETTER U WITH DIAERESIS]
 1083|      1|        '\u{0169}' => Some("u"), // ũ [LATIN SMALL LETTER U WITH TILDE]
 1084|      1|        '\u{016B}' => Some("u"), // ū [LATIN SMALL LETTER U WITH MACRON]
 1085|      1|        '\u{016D}' => Some("u"), // ŭ [LATIN SMALL LETTER U WITH BREVE]
 1086|      1|        '\u{016F}' => Some("u"), // ů [LATIN SMALL LETTER U WITH RING ABOVE]
 1087|      1|        '\u{0171}' => Some("u"), // ű [LATIN SMALL LETTER U WITH DOUBLE ACUTE]
 1088|      1|        '\u{0173}' => Some("u"), // ų [LATIN SMALL LETTER U WITH OGONEK]
 1089|      1|        '\u{01B0}' => Some("u"), // ư [LATIN SMALL LETTER U WITH HORN]
 1090|      1|        '\u{01D4}' => Some("u"), // ǔ [LATIN SMALL LETTER U WITH CARON]
 1091|      1|        '\u{01D6}' => Some("u"), /* ǖ [LATIN SMALL LETTER U WITH DIAERESIS */
 1092|       |        // AND MACRON]
 1093|      1|        '\u{01D8}' => Some("u"), /* ǘ [LATIN SMALL LETTER U WITH DIAERESIS */
 1094|       |        // AND ACUTE]
 1095|      1|        '\u{01DA}' => Some("u"), /* ǚ [LATIN SMALL LETTER U WITH DIAERESIS */
 1096|       |        // AND CARON]
 1097|      1|        '\u{01DC}' => Some("u"), /* ǜ [LATIN SMALL LETTER U WITH DIAERESIS */
 1098|       |        // AND GRAVE]
 1099|      1|        '\u{0215}' => Some("u"), // ȕ [LATIN SMALL LETTER U WITH DOUBLE GRAVE]
 1100|      1|        '\u{0217}' => Some("u"), /* ȗ [LATIN SMALL LETTER U WITH INVERTED */
 1101|       |        // BREVE]
 1102|      1|        '\u{0289}' => Some("u"), // ʉ [LATIN SMALL LETTER U BAR]
 1103|      1|        '\u{1D64}' => Some("u"), // ᵤ [LATIN SUBSCRIPT SMALL LETTER U]
 1104|      1|        '\u{1D99}' => Some("u"), /* ᶙ [LATIN SMALL LETTER U WITH RETROFLEX */
 1105|       |        // HOOK]
 1106|      1|        '\u{1E73}' => Some("u"), /* ṳ [LATIN SMALL LETTER U WITH DIAERESIS */
 1107|       |        // BELOW]
 1108|      1|        '\u{1E75}' => Some("u"), // ṵ [LATIN SMALL LETTER U WITH TILDE BELOW]
 1109|      1|        '\u{1E77}' => Some("u"), /* ṷ [LATIN SMALL LETTER U WITH CIRCUMFLEX */
 1110|       |        // BELOW]
 1111|      1|        '\u{1E79}' => Some("u"), /* ṹ [LATIN SMALL LETTER U WITH TILDE AND */
 1112|       |        // ACUTE]
 1113|      1|        '\u{1E7B}' => Some("u"), /* ṻ [LATIN SMALL LETTER U WITH MACRON AND */
 1114|       |        // DIAERESIS]
 1115|      1|        '\u{1EE5}' => Some("u"), // ụ [LATIN SMALL LETTER U WITH DOT BELOW]
 1116|      1|        '\u{1EE7}' => Some("u"), // ủ [LATIN SMALL LETTER U WITH HOOK ABOVE]
 1117|      1|        '\u{1EE9}' => Some("u"), /* ứ [LATIN SMALL LETTER U WITH HORN AND */
 1118|       |        // ACUTE]
 1119|      1|        '\u{1EEB}' => Some("u"), /* ừ [LATIN SMALL LETTER U WITH HORN AND */
 1120|       |        // GRAVE]
 1121|      1|        '\u{1EED}' => Some("u"), /* ử [LATIN SMALL LETTER U WITH HORN AND */
 1122|       |        // HOOK ABOVE]
 1123|      1|        '\u{1EEF}' => Some("u"), /* ữ [LATIN SMALL LETTER U WITH HORN AND */
 1124|       |        // TILDE]
 1125|      1|        '\u{1EF1}' => Some("u"), /* ự [LATIN SMALL LETTER U WITH HORN AND */
 1126|       |        // DOT BELOW]
 1127|      1|        '\u{24E4}' => Some("u"),   // ⓤ [CIRCLED LATIN SMALL LETTER U]
 1128|      1|        '\u{FF55}' => Some("u"),   // ｕ [FULLWIDTH LATIN SMALL LETTER U]
 1129|      1|        '\u{24B0}' => Some("(u)"), // ⒰ [PARENTHESIZED LATIN SMALL LETTER U]
 1130|      1|        '\u{1D6B}' => Some("ue"),  // ᵫ [LATIN SMALL LETTER UE]
 1131|      1|        '\u{01B2}' => Some("V"),   // Ʋ [LATIN CAPITAL LETTER V WITH HOOK]
 1132|      1|        '\u{0245}' => Some("V"),   // Ʌ [LATIN CAPITAL LETTER TURNED V]
 1133|      1|        '\u{1D20}' => Some("V"),   // ᴠ [LATIN LETTER SMALL CAPITAL V]
 1134|      1|        '\u{1E7C}' => Some("V"),   // Ṽ [LATIN CAPITAL LETTER V WITH TILDE]
 1135|      1|        '\u{1E7E}' => Some("V"),   // Ṿ [LATIN CAPITAL LETTER V WITH DOT BELOW]
 1136|      1|        '\u{1EFC}' => Some("V"),   // Ỽ [LATIN CAPITAL LETTER MIDDLE-WELSH V]
 1137|      1|        '\u{24CB}' => Some("V"),   // Ⓥ [CIRCLED LATIN CAPITAL LETTER V]
 1138|      1|        '\u{A75E}' => Some("V"),   /* Ꝟ [LATIN CAPITAL LETTER V WITH DIAGONAL */
 1139|       |        // STROKE]
 1140|      1|        '\u{A768}' => Some("V"), // Ꝩ [LATIN CAPITAL LETTER VEND]
 1141|      1|        '\u{FF36}' => Some("V"), // Ｖ [FULLWIDTH LATIN CAPITAL LETTER V]
 1142|      1|        '\u{028B}' => Some("v"), // ʋ [LATIN SMALL LETTER V WITH HOOK]
 1143|      1|        '\u{028C}' => Some("v"), // ʌ [LATIN SMALL LETTER TURNED V]
 1144|      1|        '\u{1D65}' => Some("v"), // ᵥ [LATIN SUBSCRIPT SMALL LETTER V]
 1145|      1|        '\u{1D8C}' => Some("v"), // ᶌ [LATIN SMALL LETTER V WITH PALATAL HOOK]
 1146|      1|        '\u{1E7D}' => Some("v"), // ṽ [LATIN SMALL LETTER V WITH TILDE]
 1147|      1|        '\u{1E7F}' => Some("v"), // ṿ [LATIN SMALL LETTER V WITH DOT BELOW]
 1148|      1|        '\u{24E5}' => Some("v"), // ⓥ [CIRCLED LATIN SMALL LETTER V]
 1149|      1|        '\u{2C71}' => Some("v"), /* ⱱ [LATIN SMALL LETTER V WITH ALIGN_RIGHT */
 1150|       |        // HOOK]
 1151|      1|        '\u{2C74}' => Some("v"), // ⱴ [LATIN SMALL LETTER V WITH CURL]
 1152|      1|        '\u{A75F}' => Some("v"), /* ꝟ [LATIN SMALL LETTER V WITH DIAGONAL */
 1153|       |        // STROKE]
 1154|      1|        '\u{FF56}' => Some("v"),   // ｖ [FULLWIDTH LATIN SMALL LETTER V]
 1155|      1|        '\u{A760}' => Some("VY"),  // Ꝡ [LATIN CAPITAL LETTER VY]
 1156|      1|        '\u{24B1}' => Some("(v)"), // ⒱ [PARENTHESIZED LATIN SMALL LETTER V]
 1157|      1|        '\u{A761}' => Some("vy"),  // ꝡ [LATIN SMALL LETTER VY]
 1158|      1|        '\u{0174}' => Some("W"),   // Ŵ [LATIN CAPITAL LETTER W WITH CIRCUMFLEX]
 1159|      1|        '\u{01F7}' => Some("W"),   /* Ƿ http://en.wikipedia.org/wiki/Wynn [LATIN CAPITAL LETTER WYNN] */
 1160|      1|        '\u{1D21}' => Some("W"),   // ᴡ [LATIN LETTER SMALL CAPITAL W]
 1161|      1|        '\u{1E80}' => Some("W"),   // Ẁ [LATIN CAPITAL LETTER W WITH GRAVE]
 1162|      1|        '\u{1E82}' => Some("W"),   // Ẃ [LATIN CAPITAL LETTER W WITH ACUTE]
 1163|      1|        '\u{1E84}' => Some("W"),   // Ẅ [LATIN CAPITAL LETTER W WITH DIAERESIS]
 1164|      1|        '\u{1E86}' => Some("W"),   // Ẇ [LATIN CAPITAL LETTER W WITH DOT ABOVE]
 1165|      1|        '\u{1E88}' => Some("W"),   // Ẉ [LATIN CAPITAL LETTER W WITH DOT BELOW]
 1166|      1|        '\u{24CC}' => Some("W"),   // Ⓦ [CIRCLED LATIN CAPITAL LETTER W]
 1167|      1|        '\u{2C72}' => Some("W"),   // Ⱳ [LATIN CAPITAL LETTER W WITH HOOK]
 1168|      1|        '\u{FF37}' => Some("W"),   // Ｗ [FULLWIDTH LATIN CAPITAL LETTER W]
 1169|      1|        '\u{0175}' => Some("w"),   // ŵ [LATIN SMALL LETTER W WITH CIRCUMFLEX]
 1170|      1|        '\u{01BF}' => Some("w"),   /* ƿ http://en.wikipedia.org/wiki/Wynn [LATIN LETTER WYNN] */
 1171|      1|        '\u{028D}' => Some("w"),   // ʍ [LATIN SMALL LETTER TURNED W]
 1172|      1|        '\u{1E81}' => Some("w"),   // ẁ [LATIN SMALL LETTER W WITH GRAVE]
 1173|      1|        '\u{1E83}' => Some("w"),   // ẃ [LATIN SMALL LETTER W WITH ACUTE]
 1174|      1|        '\u{1E85}' => Some("w"),   // ẅ [LATIN SMALL LETTER W WITH DIAERESIS]
 1175|      1|        '\u{1E87}' => Some("w"),   // ẇ [LATIN SMALL LETTER W WITH DOT ABOVE]
 1176|      1|        '\u{1E89}' => Some("w"),   // ẉ [LATIN SMALL LETTER W WITH DOT BELOW]
 1177|      1|        '\u{1E98}' => Some("w"),   // ẘ [LATIN SMALL LETTER W WITH RING ABOVE]
 1178|      1|        '\u{24E6}' => Some("w"),   // ⓦ [CIRCLED LATIN SMALL LETTER W]
 1179|      1|        '\u{2C73}' => Some("w"),   // ⱳ [LATIN SMALL LETTER W WITH HOOK]
 1180|      1|        '\u{FF57}' => Some("w"),   // ｗ [FULLWIDTH LATIN SMALL LETTER W]
 1181|      1|        '\u{24B2}' => Some("(w)"), // ⒲ [PARENTHESIZED LATIN SMALL LETTER W]
 1182|      1|        '\u{1E8A}' => Some("X"),   // Ẋ [LATIN CAPITAL LETTER X WITH DOT ABOVE]
 1183|      1|        '\u{1E8C}' => Some("X"),   // Ẍ [LATIN CAPITAL LETTER X WITH DIAERESIS]
 1184|      1|        '\u{24CD}' => Some("X"),   // Ⓧ [CIRCLED LATIN CAPITAL LETTER X]
 1185|      1|        '\u{FF38}' => Some("X"),   // Ｘ [FULLWIDTH LATIN CAPITAL LETTER X]
 1186|      1|        '\u{1D8D}' => Some("x"),   // ᶍ [LATIN SMALL LETTER X WITH PALATAL HOOK]
 1187|      1|        '\u{1E8B}' => Some("x"),   // ẋ [LATIN SMALL LETTER X WITH DOT ABOVE]
 1188|      1|        '\u{1E8D}' => Some("x"),   // ẍ [LATIN SMALL LETTER X WITH DIAERESIS]
 1189|      1|        '\u{2093}' => Some("x"),   // ₓ [LATIN SUBSCRIPT SMALL LETTER X]
 1190|      1|        '\u{24E7}' => Some("x"),   // ⓧ [CIRCLED LATIN SMALL LETTER X]
 1191|      1|        '\u{FF58}' => Some("x"),   // ｘ [FULLWIDTH LATIN SMALL LETTER X]
 1192|      1|        '\u{24B3}' => Some("(x)"), // ⒳ [PARENTHESIZED LATIN SMALL LETTER X]
 1193|      1|        '\u{00DD}' => Some("Y"),   // Ý [LATIN CAPITAL LETTER Y WITH ACUTE]
 1194|      1|        '\u{0176}' => Some("Y"),   // Ŷ [LATIN CAPITAL LETTER Y WITH CIRCUMFLEX]
 1195|      1|        '\u{0178}' => Some("Y"),   // Ÿ [LATIN CAPITAL LETTER Y WITH DIAERESIS]
 1196|      1|        '\u{01B3}' => Some("Y"),   // Ƴ [LATIN CAPITAL LETTER Y WITH HOOK]
 1197|      1|        '\u{0232}' => Some("Y"),   // Ȳ [LATIN CAPITAL LETTER Y WITH MACRON]
 1198|      1|        '\u{024E}' => Some("Y"),   // Ɏ [LATIN CAPITAL LETTER Y WITH STROKE]
 1199|      1|        '\u{028F}' => Some("Y"),   // ʏ [LATIN LETTER SMALL CAPITAL Y]
 1200|      1|        '\u{1E8E}' => Some("Y"),   // Ẏ [LATIN CAPITAL LETTER Y WITH DOT ABOVE]
 1201|      1|        '\u{1EF2}' => Some("Y"),   // Ỳ [LATIN CAPITAL LETTER Y WITH GRAVE]
 1202|      1|        '\u{1EF4}' => Some("Y"),   // Ỵ [LATIN CAPITAL LETTER Y WITH DOT BELOW]
 1203|      1|        '\u{1EF6}' => Some("Y"),   // Ỷ [LATIN CAPITAL LETTER Y WITH HOOK ABOVE]
 1204|      1|        '\u{1EF8}' => Some("Y"),   // Ỹ [LATIN CAPITAL LETTER Y WITH TILDE]
 1205|      1|        '\u{1EFE}' => Some("Y"),   // Ỿ [LATIN CAPITAL LETTER Y WITH LOOP]
 1206|      1|        '\u{24CE}' => Some("Y"),   // Ⓨ [CIRCLED LATIN CAPITAL LETTER Y]
 1207|      1|        '\u{FF39}' => Some("Y"),   // Ｙ [FULLWIDTH LATIN CAPITAL LETTER Y]
 1208|      1|        '\u{00FD}' => Some("y"),   // ý [LATIN SMALL LETTER Y WITH ACUTE]
 1209|      1|        '\u{00FF}' => Some("y"),   // ÿ [LATIN SMALL LETTER Y WITH DIAERESIS]
 1210|      1|        '\u{0177}' => Some("y"),   // ŷ [LATIN SMALL LETTER Y WITH CIRCUMFLEX]
 1211|      1|        '\u{01B4}' => Some("y"),   // ƴ [LATIN SMALL LETTER Y WITH HOOK]
 1212|      1|        '\u{0233}' => Some("y"),   // ȳ [LATIN SMALL LETTER Y WITH MACRON]
 1213|      1|        '\u{024F}' => Some("y"),   // ɏ [LATIN SMALL LETTER Y WITH STROKE]
 1214|      1|        '\u{028E}' => Some("y"),   // ʎ [LATIN SMALL LETTER TURNED Y]
 1215|      1|        '\u{1E8F}' => Some("y"),   // ẏ [LATIN SMALL LETTER Y WITH DOT ABOVE]
 1216|      1|        '\u{1E99}' => Some("y"),   // ẙ [LATIN SMALL LETTER Y WITH RING ABOVE]
 1217|      1|        '\u{1EF3}' => Some("y"),   // ỳ [LATIN SMALL LETTER Y WITH GRAVE]
 1218|      1|        '\u{1EF5}' => Some("y"),   // ỵ [LATIN SMALL LETTER Y WITH DOT BELOW]
 1219|      1|        '\u{1EF7}' => Some("y"),   // ỷ [LATIN SMALL LETTER Y WITH HOOK ABOVE]
 1220|      1|        '\u{1EF9}' => Some("y"),   // ỹ [LATIN SMALL LETTER Y WITH TILDE]
 1221|      1|        '\u{1EFF}' => Some("y"),   // ỿ [LATIN SMALL LETTER Y WITH LOOP]
 1222|      1|        '\u{24E8}' => Some("y"),   // ⓨ [CIRCLED LATIN SMALL LETTER Y]
 1223|      1|        '\u{FF59}' => Some("y"),   // ｙ [FULLWIDTH LATIN SMALL LETTER Y]
 1224|      1|        '\u{24B4}' => Some("(y)"), // ⒴ [PARENTHESIZED LATIN SMALL LETTER Y]
 1225|      1|        '\u{0179}' => Some("Z"),   // Ź [LATIN CAPITAL LETTER Z WITH ACUTE]
 1226|      1|        '\u{017B}' => Some("Z"),   // Ż [LATIN CAPITAL LETTER Z WITH DOT ABOVE]
 1227|      1|        '\u{017D}' => Some("Z"),   // Ž [LATIN CAPITAL LETTER Z WITH CARON]
 1228|      1|        '\u{01B5}' => Some("Z"),   // Ƶ [LATIN CAPITAL LETTER Z WITH STROKE]
 1229|      1|        '\u{021C}' => Some("Z"),   /* Ȝ http://en.wikipedia.org/wiki/Yogh [LATIN CAPITAL LETTER YOGH] */
 1230|      1|        '\u{0224}' => Some("Z"),   // Ȥ [LATIN CAPITAL LETTER Z WITH HOOK]
 1231|      1|        '\u{1D22}' => Some("Z"),   // ᴢ [LATIN LETTER SMALL CAPITAL Z]
 1232|      1|        '\u{1E90}' => Some("Z"),   // Ẑ [LATIN CAPITAL LETTER Z WITH CIRCUMFLEX]
 1233|      1|        '\u{1E92}' => Some("Z"),   // Ẓ [LATIN CAPITAL LETTER Z WITH DOT BELOW]
 1234|      1|        '\u{1E94}' => Some("Z"),   // Ẕ [LATIN CAPITAL LETTER Z WITH LINE BELOW]
 1235|      1|        '\u{24CF}' => Some("Z"),   // Ⓩ [CIRCLED LATIN CAPITAL LETTER Z]
 1236|      1|        '\u{2C6B}' => Some("Z"),   // Ⱬ [LATIN CAPITAL LETTER Z WITH DESCENDER]
 1237|      1|        '\u{A762}' => Some("Z"),   // Ꝣ [LATIN CAPITAL LETTER VISIGOTHIC Z]
 1238|      1|        '\u{FF3A}' => Some("Z"),   // Ｚ [FULLWIDTH LATIN CAPITAL LETTER Z]
 1239|      1|        '\u{017A}' => Some("z"),   // ź [LATIN SMALL LETTER Z WITH ACUTE]
 1240|      1|        '\u{017C}' => Some("z"),   // ż [LATIN SMALL LETTER Z WITH DOT ABOVE]
 1241|      1|        '\u{017E}' => Some("z"),   // ž [LATIN SMALL LETTER Z WITH CARON]
 1242|      1|        '\u{01B6}' => Some("z"),   // ƶ [LATIN SMALL LETTER Z WITH STROKE]
 1243|      1|        '\u{021D}' => Some("z"),   /* ȝ http://en.wikipedia.org/wiki/Yogh [LATIN SMALL LETTER YOGH] */
 1244|      1|        '\u{0225}' => Some("z"),   // ȥ [LATIN SMALL LETTER Z WITH HOOK]
 1245|      1|        '\u{0240}' => Some("z"),   // ɀ [LATIN SMALL LETTER Z WITH SWASH TAIL]
 1246|      1|        '\u{0290}' => Some("z"),   /* ʐ [LATIN SMALL LETTER Z WITH RETROFLEX */
 1247|       |        // HOOK]
 1248|      1|        '\u{0291}' => Some("z"),   // ʑ [LATIN SMALL LETTER Z WITH CURL]
 1249|      1|        '\u{1D76}' => Some("z"),   // ᵶ [LATIN SMALL LETTER Z WITH MIDDLE TILDE]
 1250|      1|        '\u{1D8E}' => Some("z"),   // ᶎ [LATIN SMALL LETTER Z WITH PALATAL HOOK]
 1251|      1|        '\u{1E91}' => Some("z"),   // ẑ [LATIN SMALL LETTER Z WITH CIRCUMFLEX]
 1252|      1|        '\u{1E93}' => Some("z"),   // ẓ [LATIN SMALL LETTER Z WITH DOT BELOW]
 1253|      1|        '\u{1E95}' => Some("z"),   // ẕ [LATIN SMALL LETTER Z WITH LINE BELOW]
 1254|      1|        '\u{24E9}' => Some("z"),   // ⓩ [CIRCLED LATIN SMALL LETTER Z]
 1255|      1|        '\u{2C6C}' => Some("z"),   // ⱬ [LATIN SMALL LETTER Z WITH DESCENDER]
 1256|      1|        '\u{A763}' => Some("z"),   // ꝣ [LATIN SMALL LETTER VISIGOTHIC Z]
 1257|      1|        '\u{FF5A}' => Some("z"),   // ｚ [FULLWIDTH LATIN SMALL LETTER Z]
 1258|      1|        '\u{24B5}' => Some("(z)"), // ⒵ [PARENTHESIZED LATIN SMALL LETTER Z]
 1259|      1|        '\u{2070}' => Some("0"),   // ⁰ [SUPERSCRIPT ZERO]
 1260|      1|        '\u{2080}' => Some("0"),   // ₀ [SUBSCRIPT ZERO]
 1261|      1|        '\u{24EA}' => Some("0"),   // ⓪ [CIRCLED DIGIT ZERO]
 1262|      1|        '\u{24FF}' => Some("0"),   // ⓿ [NEGATIVE CIRCLED DIGIT ZERO]
 1263|      1|        '\u{FF10}' => Some("0"),   // ０ [FULLWIDTH DIGIT ZERO]
 1264|      1|        '\u{00B9}' => Some("1"),   // ¹ [SUPERSCRIPT ONE]
 1265|      1|        '\u{2081}' => Some("1"),   // ₁ [SUBSCRIPT ONE]
 1266|      1|        '\u{2460}' => Some("1"),   // ① [CIRCLED DIGIT ONE]
 1267|      1|        '\u{24F5}' => Some("1"),   // ⓵ [DOUBLE CIRCLED DIGIT ONE]
 1268|      1|        '\u{2776}' => Some("1"),   // ❶ [DINGBAT NEGATIVE CIRCLED DIGIT ONE]
 1269|      1|        '\u{2780}' => Some("1"),   // ➀ [DINGBAT CIRCLED SANS-SERIF DIGIT ONE]
 1270|      1|        '\u{278A}' => Some("1"),   /* ➊ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1271|       |        // DIGIT ONE]
 1272|      1|        '\u{FF11}' => Some("1"),   // １ [FULLWIDTH DIGIT ONE]
 1273|      1|        '\u{2488}' => Some("1."),  // ⒈ [DIGIT ONE FULL STOP]
 1274|      1|        '\u{2474}' => Some("(1)"), // ⑴ [PARENTHESIZED DIGIT ONE]
 1275|      1|        '\u{00B2}' => Some("2"),   // ² [SUPERSCRIPT TWO]
 1276|      1|        '\u{2082}' => Some("2"),   // ₂ [SUBSCRIPT TWO]
 1277|      1|        '\u{2461}' => Some("2"),   // ② [CIRCLED DIGIT TWO]
 1278|      1|        '\u{24F6}' => Some("2"),   // ⓶ [DOUBLE CIRCLED DIGIT TWO]
 1279|      1|        '\u{2777}' => Some("2"),   // ❷ [DINGBAT NEGATIVE CIRCLED DIGIT TWO]
 1280|      1|        '\u{2781}' => Some("2"),   // ➁ [DINGBAT CIRCLED SANS-SERIF DIGIT TWO]
 1281|      1|        '\u{278B}' => Some("2"),   /* ➋ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1282|       |        // DIGIT TWO]
 1283|      1|        '\u{FF12}' => Some("2"),   // ２ [FULLWIDTH DIGIT TWO]
 1284|      1|        '\u{2489}' => Some("2."),  // ⒉ [DIGIT TWO FULL STOP]
 1285|      1|        '\u{2475}' => Some("(2)"), // ⑵ [PARENTHESIZED DIGIT TWO]
 1286|      1|        '\u{00B3}' => Some("3"),   // ³ [SUPERSCRIPT THREE]
 1287|      1|        '\u{2083}' => Some("3"),   // ₃ [SUBSCRIPT THREE]
 1288|      1|        '\u{2462}' => Some("3"),   // ③ [CIRCLED DIGIT THREE]
 1289|      1|        '\u{24F7}' => Some("3"),   // ⓷ [DOUBLE CIRCLED DIGIT THREE]
 1290|      1|        '\u{2778}' => Some("3"),   // ❸ [DINGBAT NEGATIVE CIRCLED DIGIT THREE]
 1291|      1|        '\u{2782}' => Some("3"),   // ➂ [DINGBAT CIRCLED SANS-SERIF DIGIT THREE]
 1292|      1|        '\u{278C}' => Some("3"),   /* ➌ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1293|       |        // DIGIT THREE]
 1294|      1|        '\u{FF13}' => Some("3"),   // ３ [FULLWIDTH DIGIT THREE]
 1295|      1|        '\u{248A}' => Some("3."),  // ⒊ [DIGIT THREE FULL STOP]
 1296|      1|        '\u{2476}' => Some("(3)"), // ⑶ [PARENTHESIZED DIGIT THREE]
 1297|      1|        '\u{2074}' => Some("4"),   // ⁴ [SUPERSCRIPT FOUR]
 1298|      1|        '\u{2084}' => Some("4"),   // ₄ [SUBSCRIPT FOUR]
 1299|      1|        '\u{2463}' => Some("4"),   // ④ [CIRCLED DIGIT FOUR]
 1300|      1|        '\u{24F8}' => Some("4"),   // ⓸ [DOUBLE CIRCLED DIGIT FOUR]
 1301|      1|        '\u{2779}' => Some("4"),   // ❹ [DINGBAT NEGATIVE CIRCLED DIGIT FOUR]
 1302|      1|        '\u{2783}' => Some("4"),   // ➃ [DINGBAT CIRCLED SANS-SERIF DIGIT FOUR]
 1303|      1|        '\u{278D}' => Some("4"),   /* ➍ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1304|       |        // DIGIT FOUR]
 1305|      1|        '\u{FF14}' => Some("4"),   // ４ [FULLWIDTH DIGIT FOUR]
 1306|      1|        '\u{248B}' => Some("4."),  // ⒋ [DIGIT FOUR FULL STOP]
 1307|      1|        '\u{2477}' => Some("(4)"), // ⑷ [PARENTHESIZED DIGIT FOUR]
 1308|      1|        '\u{2075}' => Some("5"),   // ⁵ [SUPERSCRIPT FIVE]
 1309|      1|        '\u{2085}' => Some("5"),   // ₅ [SUBSCRIPT FIVE]
 1310|      1|        '\u{2464}' => Some("5"),   // ⑤ [CIRCLED DIGIT FIVE]
 1311|      1|        '\u{24F9}' => Some("5"),   // ⓹ [DOUBLE CIRCLED DIGIT FIVE]
 1312|      1|        '\u{277A}' => Some("5"),   // ❺ [DINGBAT NEGATIVE CIRCLED DIGIT FIVE]
 1313|      1|        '\u{2784}' => Some("5"),   // ➄ [DINGBAT CIRCLED SANS-SERIF DIGIT FIVE]
 1314|      1|        '\u{278E}' => Some("5"),   /* ➎ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1315|       |        // DIGIT FIVE]
 1316|      1|        '\u{FF15}' => Some("5"),   // ５ [FULLWIDTH DIGIT FIVE]
 1317|      1|        '\u{248C}' => Some("5."),  // ⒌ [DIGIT FIVE FULL STOP]
 1318|      1|        '\u{2478}' => Some("(5)"), // ⑸ [PARENTHESIZED DIGIT FIVE]
 1319|      1|        '\u{2076}' => Some("6"),   // ⁶ [SUPERSCRIPT SIX]
 1320|      1|        '\u{2086}' => Some("6"),   // ₆ [SUBSCRIPT SIX]
 1321|      1|        '\u{2465}' => Some("6"),   // ⑥ [CIRCLED DIGIT SIX]
 1322|      1|        '\u{24FA}' => Some("6"),   // ⓺ [DOUBLE CIRCLED DIGIT SIX]
 1323|      1|        '\u{277B}' => Some("6"),   // ❻ [DINGBAT NEGATIVE CIRCLED DIGIT SIX]
 1324|      1|        '\u{2785}' => Some("6"),   // ➅ [DINGBAT CIRCLED SANS-SERIF DIGIT SIX]
 1325|      1|        '\u{278F}' => Some("6"),   /* ➏ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1326|       |        // DIGIT SIX]
 1327|      1|        '\u{FF16}' => Some("6"),   // ６ [FULLWIDTH DIGIT SIX]
 1328|      1|        '\u{248D}' => Some("6."),  // ⒍ [DIGIT SIX FULL STOP]
 1329|      1|        '\u{2479}' => Some("(6)"), // ⑹ [PARENTHESIZED DIGIT SIX]
 1330|      1|        '\u{2077}' => Some("7"),   // ⁷ [SUPERSCRIPT SEVEN]
 1331|      1|        '\u{2087}' => Some("7"),   // ₇ [SUBSCRIPT SEVEN]
 1332|      1|        '\u{2466}' => Some("7"),   // ⑦ [CIRCLED DIGIT SEVEN]
 1333|      1|        '\u{24FB}' => Some("7"),   // ⓻ [DOUBLE CIRCLED DIGIT SEVEN]
 1334|      1|        '\u{277C}' => Some("7"),   // ❼ [DINGBAT NEGATIVE CIRCLED DIGIT SEVEN]
 1335|      1|        '\u{2786}' => Some("7"),   // ➆ [DINGBAT CIRCLED SANS-SERIF DIGIT SEVEN]
 1336|      1|        '\u{2790}' => Some("7"),   /* ➐ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1337|       |        // DIGIT SEVEN]
 1338|      1|        '\u{FF17}' => Some("7"),   // ７ [FULLWIDTH DIGIT SEVEN]
 1339|      1|        '\u{248E}' => Some("7."),  // ⒎ [DIGIT SEVEN FULL STOP]
 1340|      1|        '\u{247A}' => Some("(7)"), // ⑺ [PARENTHESIZED DIGIT SEVEN]
 1341|      1|        '\u{2078}' => Some("8"),   // ⁸ [SUPERSCRIPT EIGHT]
 1342|      1|        '\u{2088}' => Some("8"),   // ₈ [SUBSCRIPT EIGHT]
 1343|      1|        '\u{2467}' => Some("8"),   // ⑧ [CIRCLED DIGIT EIGHT]
 1344|      1|        '\u{24FC}' => Some("8"),   // ⓼ [DOUBLE CIRCLED DIGIT EIGHT]
 1345|      1|        '\u{277D}' => Some("8"),   // ❽ [DINGBAT NEGATIVE CIRCLED DIGIT EIGHT]
 1346|      1|        '\u{2787}' => Some("8"),   // ➇ [DINGBAT CIRCLED SANS-SERIF DIGIT EIGHT]
 1347|      1|        '\u{2791}' => Some("8"),   /* ➑ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1348|       |        // DIGIT EIGHT]
 1349|      1|        '\u{FF18}' => Some("8"),   // ８ [FULLWIDTH DIGIT EIGHT]
 1350|      1|        '\u{248F}' => Some("8."),  // ⒏ [DIGIT EIGHT FULL STOP]
 1351|      1|        '\u{247B}' => Some("(8)"), // ⑻ [PARENTHESIZED DIGIT EIGHT]
 1352|      1|        '\u{2079}' => Some("9"),   // ⁹ [SUPERSCRIPT NINE]
 1353|      1|        '\u{2089}' => Some("9"),   // ₉ [SUBSCRIPT NINE]
 1354|      1|        '\u{2468}' => Some("9"),   // ⑨ [CIRCLED DIGIT NINE]
 1355|      1|        '\u{24FD}' => Some("9"),   // ⓽ [DOUBLE CIRCLED DIGIT NINE]
 1356|      1|        '\u{277E}' => Some("9"),   // ❾ [DINGBAT NEGATIVE CIRCLED DIGIT NINE]
 1357|      1|        '\u{2788}' => Some("9"),   // ➈ [DINGBAT CIRCLED SANS-SERIF DIGIT NINE]
 1358|      1|        '\u{2792}' => Some("9"),   /* ➒ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1359|       |        // DIGIT NINE]
 1360|      1|        '\u{FF19}' => Some("9"),   // ９ [FULLWIDTH DIGIT NINE]
 1361|      1|        '\u{2490}' => Some("9."),  // ⒐ [DIGIT NINE FULL STOP]
 1362|      1|        '\u{247C}' => Some("(9)"), // ⑼ [PARENTHESIZED DIGIT NINE]
 1363|      1|        '\u{2469}' => Some("10"),  // ⑩ [CIRCLED NUMBER TEN]
 1364|      1|        '\u{24FE}' => Some("10"),  // ⓾ [DOUBLE CIRCLED NUMBER TEN]
 1365|      1|        '\u{277F}' => Some("10"),  // ❿ [DINGBAT NEGATIVE CIRCLED NUMBER TEN]
 1366|      1|        '\u{2789}' => Some("10"),  // ➉ [DINGBAT CIRCLED SANS-SERIF NUMBER TEN]
 1367|      1|        '\u{2793}' => Some("10"),  /* ➓ [DINGBAT NEGATIVE CIRCLED SANS-SERIF */
 1368|       |        // NUMBER TEN]
 1369|      2|        '\u{2491}' => Some("10."),  // ⒑ [NUMBER TEN FULL STOP]
 1370|      2|        '\u{247D}' => Some("(10)"), // ⑽ [PARENTHESIZED NUMBER TEN]
 1371|      1|        '\u{246A}' => Some("11"),   // ⑪ [CIRCLED NUMBER ELEVEN]
 1372|      1|        '\u{24EB}' => Some("11"),   // ⓫ [NEGATIVE CIRCLED NUMBER ELEVEN]
 1373|      1|        '\u{2492}' => Some("11."),  // ⒒ [NUMBER ELEVEN FULL STOP]
 1374|      1|        '\u{247E}' => Some("(11)"), // ⑾ [PARENTHESIZED NUMBER ELEVEN]
 1375|      1|        '\u{246B}' => Some("12"),   // ⑫ [CIRCLED NUMBER TWELVE]
 1376|      1|        '\u{24EC}' => Some("12"),   // ⓬ [NEGATIVE CIRCLED NUMBER TWELVE]
 1377|      1|        '\u{2493}' => Some("12."),  // ⒓ [NUMBER TWELVE FULL STOP]
 1378|      1|        '\u{247F}' => Some("(12)"), // ⑿ [PARENTHESIZED NUMBER TWELVE]
 1379|      1|        '\u{246C}' => Some("13"),   // ⑬ [CIRCLED NUMBER THIRTEEN]
 1380|      1|        '\u{24ED}' => Some("13"),   // ⓭ [NEGATIVE CIRCLED NUMBER THIRTEEN]
 1381|      1|        '\u{2494}' => Some("13."),  // ⒔ [NUMBER THIRTEEN FULL STOP]
 1382|      1|        '\u{2480}' => Some("(13)"), // ⒀ [PARENTHESIZED NUMBER THIRTEEN]
 1383|      1|        '\u{246D}' => Some("14"),   // ⑭ [CIRCLED NUMBER FOURTEEN]
 1384|      1|        '\u{24EE}' => Some("14"),   // ⓮ [NEGATIVE CIRCLED NUMBER FOURTEEN]
 1385|      1|        '\u{2495}' => Some("14."),  // ⒕ [NUMBER FOURTEEN FULL STOP]
 1386|      1|        '\u{2481}' => Some("(14)"), // ⒁ [PARENTHESIZED NUMBER FOURTEEN]
 1387|      1|        '\u{246E}' => Some("15"),   // ⑮ [CIRCLED NUMBER FIFTEEN]
 1388|      1|        '\u{24EF}' => Some("15"),   // ⓯ [NEGATIVE CIRCLED NUMBER FIFTEEN]
 1389|      1|        '\u{2496}' => Some("15."),  // ⒖ [NUMBER FIFTEEN FULL STOP]
 1390|      1|        '\u{2482}' => Some("(15)"), // ⒂ [PARENTHESIZED NUMBER FIFTEEN]
 1391|      1|        '\u{246F}' => Some("16"),   // ⑯ [CIRCLED NUMBER SIXTEEN]
 1392|      1|        '\u{24F0}' => Some("16"),   // ⓰ [NEGATIVE CIRCLED NUMBER SIXTEEN]
 1393|      1|        '\u{2497}' => Some("16."),  // ⒗ [NUMBER SIXTEEN FULL STOP]
 1394|      1|        '\u{2483}' => Some("(16)"), // ⒃ [PARENTHESIZED NUMBER SIXTEEN]
 1395|      1|        '\u{2470}' => Some("17"),   // ⑰ [CIRCLED NUMBER SEVENTEEN]
 1396|      1|        '\u{24F1}' => Some("17"),   // ⓱ [NEGATIVE CIRCLED NUMBER SEVENTEEN]
 1397|      1|        '\u{2498}' => Some("17."),  // ⒘ [NUMBER SEVENTEEN FULL STOP]
 1398|      1|        '\u{2484}' => Some("(17)"), // ⒄ [PARENTHESIZED NUMBER SEVENTEEN]
 1399|      1|        '\u{2471}' => Some("18"),   // ⑱ [CIRCLED NUMBER EIGHTEEN]
 1400|      1|        '\u{24F2}' => Some("18"),   // ⓲ [NEGATIVE CIRCLED NUMBER EIGHTEEN]
 1401|      1|        '\u{2499}' => Some("18."),  // ⒙ [NUMBER EIGHTEEN FULL STOP]
 1402|      1|        '\u{2485}' => Some("(18)"), // ⒅ [PARENTHESIZED NUMBER EIGHTEEN]
 1403|      1|        '\u{2472}' => Some("19"),   // ⑲ [CIRCLED NUMBER NINETEEN]
 1404|      1|        '\u{24F3}' => Some("19"),   // ⓳ [NEGATIVE CIRCLED NUMBER NINETEEN]
 1405|      1|        '\u{249A}' => Some("19."),  // ⒚ [NUMBER NINETEEN FULL STOP]
 1406|      1|        '\u{2486}' => Some("(19)"), // ⒆ [PARENTHESIZED NUMBER NINETEEN]
 1407|      1|        '\u{2473}' => Some("20"),   // ⑳ [CIRCLED NUMBER TWENTY]
 1408|      1|        '\u{24F4}' => Some("20"),   // ⓴ [NEGATIVE CIRCLED NUMBER TWENTY]
 1409|      1|        '\u{249B}' => Some("20."),  // ⒛ [NUMBER TWENTY FULL STOP]
 1410|      1|        '\u{2487}' => Some("(20)"), // ⒇ [PARENTHESIZED NUMBER TWENTY]
 1411|      1|        '\u{00AB}' => Some("\""),   /* « [ALIGN_LEFT-POINTING DOUBLE ANGLE */
 1412|       |        // QUOTATION MARK]
 1413|      1|        '\u{00BB}' => Some("\""), /* » [ALIGN_RIGHT-POINTING DOUBLE ANGLE */
 1414|       |        // QUOTATION MARK]
 1415|      1|        '\u{201C}' => Some("\""), // “ [ALIGN_LEFT DOUBLE QUOTATION MARK]
 1416|      1|        '\u{201D}' => Some("\""), // ” [ALIGN_RIGHT DOUBLE QUOTATION MARK]
 1417|      1|        '\u{201E}' => Some("\""), // „ [DOUBLE LOW-9 QUOTATION MARK]
 1418|      1|        '\u{2033}' => Some("\""), // ″ [DOUBLE PRIME]
 1419|      1|        '\u{2036}' => Some("\""), // ‶ [REVERSED DOUBLE PRIME]
 1420|      1|        '\u{275D}' => Some("\""), /* ❝ [HEAVY DOUBLE TURNED COMMA QUOTATION */
 1421|       |        // MARK ORNAMENT]
 1422|      1|        '\u{275E}' => Some("\""), /* ❞ [HEAVY DOUBLE COMMA QUOTATION MARK */
 1423|       |        // ORNAMENT]
 1424|      1|        '\u{276E}' => Some("\""), /* ❮ [HEAVY ALIGN_LEFT-POINTING ANGLE */
 1425|       |        // QUOTATION MARK ORNAMENT]
 1426|      1|        '\u{276F}' => Some("\""), /* ❯ [HEAVY ALIGN_RIGHT-POINTING ANGLE */
 1427|       |        // QUOTATION MARK ORNAMENT]
 1428|      1|        '\u{FF02}' => Some("\""), // ＂ [FULLWIDTH QUOTATION MARK]
 1429|      1|        '\u{2018}' => Some("'"),  // ‘ [ALIGN_LEFT SINGLE QUOTATION MARK]
 1430|      1|        '\u{2019}' => Some("'"),  // ’ [ALIGN_RIGHT SINGLE QUOTATION MARK]
 1431|      1|        '\u{201A}' => Some("'"),  // ‚ [SINGLE LOW-9 QUOTATION MARK]
 1432|      1|        '\u{201B}' => Some("'"),  // ‛ [SINGLE HIGH-REVERSED-9 QUOTATION MARK]
 1433|      1|        '\u{2032}' => Some("'"),  // ′ [PRIME]
 1434|      1|        '\u{2035}' => Some("'"),  // ‵ [REVERSED PRIME]
 1435|      1|        '\u{2039}' => Some("'"),  /* ‹ [SINGLE ALIGN_LEFT-POINTING ANGLE */
 1436|       |        // QUOTATION MARK]
 1437|      1|        '\u{203A}' => Some("'"), /* › [SINGLE ALIGN_RIGHT-POINTING ANGLE */
 1438|       |        // QUOTATION MARK]
 1439|      1|        '\u{275B}' => Some("'"), /* ❛ [HEAVY SINGLE TURNED COMMA QUOTATION */
 1440|       |        // MARK ORNAMENT]
 1441|      1|        '\u{275C}' => Some("'"), /* ❜ [HEAVY SINGLE COMMA QUOTATION MARK */
 1442|       |        // ORNAMENT]
 1443|      1|        '\u{FF07}' => Some("'"), // ＇ [FULLWIDTH APOSTROPHE]
 1444|      1|        '\u{2010}' => Some("-"), // ‐ [HYPHEN]
 1445|      1|        '\u{2011}' => Some("-"), // ‑ [NON-BREAKING HYPHEN]
 1446|      1|        '\u{2012}' => Some("-"), // ‒ [FIGURE DASH]
 1447|      1|        '\u{2013}' => Some("-"), // – [EN DASH]
 1448|      1|        '\u{2014}' => Some("-"), // — [EM DASH]
 1449|      1|        '\u{207B}' => Some("-"), // ⁻ [SUPERSCRIPT MINUS]
 1450|      1|        '\u{208B}' => Some("-"), // ₋ [SUBSCRIPT MINUS]
 1451|      1|        '\u{FF0D}' => Some("-"), // － [FULLWIDTH HYPHEN-MINUS]
 1452|      1|        '\u{2045}' => Some("["), // ⁅ [ALIGN_LEFT SQUARE BRACKET WITH QUILL]
 1453|      1|        '\u{2772}' => Some("["), /* ❲ [LIGHT ALIGN_LEFT TORTOISE SHELL */
 1454|       |        // BRACKET ORNAMENT]
 1455|      1|        '\u{FF3B}' => Some("["), // ［ [FULLWIDTH ALIGN_LEFT SQUARE BRACKET]
 1456|      1|        '\u{2046}' => Some("]"), // ⁆ [ALIGN_RIGHT SQUARE BRACKET WITH QUILL]
 1457|      1|        '\u{2773}' => Some("]"), /* ❳ [LIGHT ALIGN_RIGHT TORTOISE SHELL */
 1458|       |        // BRACKET ORNAMENT]
 1459|      1|        '\u{FF3D}' => Some("]"), // ］ [FULLWIDTH ALIGN_RIGHT SQUARE BRACKET]
 1460|      1|        '\u{207D}' => Some("("), // ⁽ [SUPERSCRIPT ALIGN_LEFT PARENTHESIS]
 1461|      1|        '\u{208D}' => Some("("), // ₍ [SUBSCRIPT ALIGN_LEFT PARENTHESIS]
 1462|      1|        '\u{2768}' => Some("("), // ❨ [MEDIUM ALIGN_LEFT PARENTHESIS ORNAMENT]
 1463|      1|        '\u{276A}' => Some("("), /* ❪ [MEDIUM FLATTENED ALIGN_LEFT */
 1464|       |        // PARENTHESIS ORNAMENT]
 1465|      1|        '\u{FF08}' => Some("("),  // （ [FULLWIDTH ALIGN_LEFT PARENTHESIS]
 1466|      1|        '\u{2E28}' => Some("(("), // ⸨ [ALIGN_LEFT DOUBLE PARENTHESIS]
 1467|      1|        '\u{207E}' => Some(")"),  // ⁾ [SUPERSCRIPT ALIGN_RIGHT PARENTHESIS]
 1468|      1|        '\u{208E}' => Some(")"),  // ₎ [SUBSCRIPT ALIGN_RIGHT PARENTHESIS]
 1469|      1|        '\u{2769}' => Some(")"),  // ❩ [MEDIUM ALIGN_RIGHT PARENTHESIS ORNAMENT]
 1470|      1|        '\u{276B}' => Some(")"),  /* ❫ [MEDIUM FLATTENED ALIGN_RIGHT */
 1471|       |        // PARENTHESIS ORNAMENT]
 1472|      1|        '\u{FF09}' => Some(")"),  // ） [FULLWIDTH ALIGN_RIGHT PARENTHESIS]
 1473|      1|        '\u{2E29}' => Some("))"), // ⸩ [ALIGN_RIGHT DOUBLE PARENTHESIS]
 1474|      1|        '\u{276C}' => Some("<"),  /* ❬ [MEDIUM ALIGN_LEFT-POINTING ANGLE */
 1475|       |        // BRACKET ORNAMENT]
 1476|      1|        '\u{2770}' => Some("<"), /* ❰ [HEAVY ALIGN_LEFT-POINTING ANGLE */
 1477|       |        // BRACKET ORNAMENT]
 1478|      1|        '\u{FF1C}' => Some("<"), // ＜ [FULLWIDTH LESS-THAN SIGN]
 1479|      1|        '\u{276D}' => Some(">"), /* ❭ [MEDIUM ALIGN_RIGHT-POINTING ANGLE */
 1480|       |        // BRACKET ORNAMENT]
 1481|      1|        '\u{2771}' => Some(">"), /* ❱ [HEAVY ALIGN_RIGHT-POINTING ANGLE */
 1482|       |        // BRACKET ORNAMENT]
 1483|      1|        '\u{FF1E}' => Some(">"), // ＞ [FULLWIDTH GREATER-THAN SIGN]
 1484|      1|        '\u{2774}' => Some("{"), /* ❴ [MEDIUM ALIGN_LEFT CURLY BRACKET */
 1485|       |        // ORNAMENT]
 1486|      1|        '\u{FF5B}' => Some("{"), // ｛ [FULLWIDTH ALIGN_LEFT CURLY BRACKET]
 1487|      1|        '\u{2775}' => Some("}"), /* ❵ [MEDIUM ALIGN_RIGHT CURLY BRACKET */
 1488|       |        // ORNAMENT]
 1489|      1|        '\u{FF5D}' => Some("}"),  // ｝ [FULLWIDTH ALIGN_RIGHT CURLY BRACKET]
 1490|      1|        '\u{207A}' => Some("+"),  // ⁺ [SUPERSCRIPT PLUS SIGN]
 1491|      1|        '\u{208A}' => Some("+"),  // ₊ [SUBSCRIPT PLUS SIGN]
 1492|      1|        '\u{FF0B}' => Some("+"),  // ＋ [FULLWIDTH PLUS SIGN]
 1493|      1|        '\u{207C}' => Some("="),  // ⁼ [SUPERSCRIPT EQUALS SIGN]
 1494|      1|        '\u{208C}' => Some("="),  // ₌ [SUBSCRIPT EQUALS SIGN]
 1495|      1|        '\u{FF1D}' => Some("="),  // ＝ [FULLWIDTH EQUALS SIGN]
 1496|      1|        '\u{FF01}' => Some("!"),  // ！ [FULLWIDTH EXCLAMATION MARK]
 1497|      1|        '\u{203C}' => Some("!!"), // ‼ [DOUBLE EXCLAMATION MARK]
 1498|      1|        '\u{2049}' => Some("!?"), // ⁉ [EXCLAMATION QUESTION MARK]
 1499|      1|        '\u{FF03}' => Some("#"),  // ＃ [FULLWIDTH NUMBER SIGN]
 1500|      1|        '\u{FF04}' => Some("$"),  // ＄ [FULLWIDTH DOLLAR SIGN]
 1501|      1|        '\u{2052}' => Some("%"),  // ⁒ [COMMERCIAL MINUS SIGN]
 1502|      1|        '\u{FF05}' => Some("%"),  // ％ [FULLWIDTH PERCENT SIGN]
 1503|      1|        '\u{FF06}' => Some("&"),  // ＆ [FULLWIDTH AMPERSAND]
 1504|      1|        '\u{204E}' => Some("*"),  // ⁎ [LOW ASTERISK]
 1505|      1|        '\u{FF0A}' => Some("*"),  // ＊ [FULLWIDTH ASTERISK]
 1506|      1|        '\u{FF0C}' => Some(","),  // ， [FULLWIDTH COMMA]
 1507|      1|        '\u{FF0E}' => Some("."),  // ． [FULLWIDTH FULL STOP]
 1508|      1|        '\u{2044}' => Some("/"),  // ⁄ [FRACTION SLASH]
 1509|      1|        '\u{FF0F}' => Some("/"),  // ／ [FULLWIDTH SOLIDUS]
 1510|      1|        '\u{FF1A}' => Some(":"),  // ： [FULLWIDTH COLON]
 1511|      1|        '\u{204F}' => Some(";"),  // ⁏ [REVERSED SEMICOLON]
 1512|      1|        '\u{FF1B}' => Some(";"),  // ； [FULLWIDTH SEMICOLON]
 1513|      1|        '\u{FF1F}' => Some("?"),  // ？ [FULLWIDTH QUESTION MARK]
 1514|      1|        '\u{2047}' => Some("??"), // ⁇ [DOUBLE QUESTION MARK]
 1515|      1|        '\u{2048}' => Some("?!"), // ⁈ [QUESTION EXCLAMATION MARK]
 1516|      1|        '\u{FF20}' => Some("@"),  // ＠ [FULLWIDTH COMMERCIAL AT]
 1517|      1|        '\u{FF3C}' => Some("\\"), // ＼ [FULLWIDTH REVERSE SOLIDUS]
 1518|      1|        '\u{2038}' => Some("^"),  // ‸ [CARET]
 1519|      1|        '\u{FF3E}' => Some("^"),  // ＾ [FULLWIDTH CIRCUMFLEX ACCENT]
 1520|      1|        '\u{FF3F}' => Some("_"),  // ＿ [FULLWIDTH LOW LINE]
 1521|      1|        '\u{2053}' => Some("~"),  // ⁓ [SWUNG DASH]
 1522|      1|        '\u{FF5E}' => Some("~"),  // ～ [FULLWIDTH TILDE]
 1523|      1|        _ => None,
 1524|       |    }
 1525|  1.24k|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/charset.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::fmt;
    9|       |use core::hash::Hash;
   10|       |use core::hash::Hasher;
   11|       |use std::sync::OnceLock;
   12|       |use std::sync::RwLock;
   13|       |use std::sync::RwLockReadGuard;
   14|       |use std::sync::RwLockWriteGuard;
   15|       |
   16|       |use qubit_codec::ByteOrder;
   17|       |#[cfg(feature = "serde")]
   18|       |use serde::Deserialize;
   19|       |#[cfg(feature = "serde")]
   20|       |use serde::Deserializer;
   21|       |#[cfg(feature = "serde")]
   22|       |use serde::Serialize;
   23|       |#[cfg(feature = "serde")]
   24|       |use serde::Serializer;
   25|       |#[cfg(feature = "serde")]
   26|       |use serde::de;
   27|       |
   28|       |use super::charset_registration_error::CharsetRegistrationError;
   29|       |use super::internal::LabelNormalization;
   30|       |use crate::normalize_label_loose;
   31|       |use crate::normalize_label_whatwg;
   32|       |
   33|       |/// Global runtime charset registry.
   34|       |static CHARSET_REGISTRY: OnceLock<RwLock<Vec<Charset>>> = OnceLock::new();
   35|       |
   36|       |/// Identifies the charset associated with a codec or error.
   37|       |///
   38|       |/// A charset is represented by a stable normalized identifier, a display
   39|       |/// name, and accepted aliases. Equality and hashing use only the identifier, so
   40|       |/// display names and alias lists can evolve without changing identity.
   41|       |/// [`fmt::Display`] formats the human-readable name; diagnostics that need a
   42|       |/// stable cross-tool token should use [`Charset::id`].
   43|       |///
   44|       |/// # Examples
   45|       |///
   46|       |/// ```rust
   47|       |/// use qubit_codec_text::Charset;
   48|       |///
   49|       |/// const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936"]);
   50|       |///
   51|       |/// assert!(GBK.matches_label("CP936"));
   52|       |/// assert_eq!(GBK, Charset::new_static("gbk", "Chinese GBK", &[]));
   53|       |/// assert_eq!("GBK", GBK.to_string());
   54|       |/// ```
   55|       |#[derive(Clone, Copy, Debug)]
   56|       |pub struct Charset {
   57|       |    /// Stable normalized identifier used for identity comparison.
   58|       |    id: &'static str,
   59|       |    /// Human-friendly display name for logs and errors.
   60|       |    name: &'static str,
   61|       |    /// Static alias list accepted in label matching.
   62|       |    aliases: &'static [&'static str],
   63|       |}
   64|       |
   65|       |impl Charset {
   66|       |    /// US-ASCII text.
   67|       |    pub const ASCII: Self = Self::new_static("ascii", "ASCII", &["us-ascii"]);
   68|       |
   69|       |    /// ISO-8859-1 / Latin-1 text.
   70|       |    pub const ISO_8859_1: Self = Self::new_static(
   71|       |        "iso-8859-1",
   72|       |        "ISO-8859-1",
   73|       |        &["latin1", "latin-1", "iso8859-1", "csisolatin1", "iso_8859-1"],
   74|       |    );
   75|       |
   76|       |    /// UTF-8 text.
   77|       |    pub const UTF_8: Self = Self::new_static("utf-8", "UTF-8", &["utf8"]);
   78|       |
   79|       |    /// UTF-16 text.
   80|       |    pub const UTF_16: Self = Self::new_static("utf-16", "UTF-16", &["utf16"]);
   81|       |
   82|       |    /// UTF-16 text serialized in little-endian byte order.
   83|       |    pub const UTF_16LE: Self = Self::new_static("utf-16le", "UTF-16LE", &["utf16le", "utf16_le", "utf_16_le"]);
   84|       |
   85|       |    /// UTF-16 text serialized in big-endian byte order.
   86|       |    pub const UTF_16BE: Self = Self::new_static("utf-16be", "UTF-16BE", &["utf16be", "utf16_be", "utf_16_be"]);
   87|       |
   88|       |    /// UTF-32 text.
   89|       |    pub const UTF_32: Self = Self::new_static("utf-32", "UTF-32", &["utf32"]);
   90|       |
   91|       |    /// UTF-32 text serialized in little-endian byte order.
   92|       |    pub const UTF_32LE: Self = Self::new_static("utf-32le", "UTF-32LE", &["utf32le", "utf32_le", "utf_32_le"]);
   93|       |
   94|       |    /// UTF-32 text serialized in big-endian byte order.
   95|       |    pub const UTF_32BE: Self = Self::new_static("utf-32be", "UTF-32BE", &["utf32be", "utf32_be", "utf_32_be"]);
   96|       |
   97|       |    /// Built-in charsets known by this crate.
   98|       |    pub const BUILTINS: &'static [Self] = &[
   99|       |        Self::ASCII,
  100|       |        Self::ISO_8859_1,
  101|       |        Self::UTF_8,
  102|       |        Self::UTF_16,
  103|       |        Self::UTF_16LE,
  104|       |        Self::UTF_16BE,
  105|       |        Self::UTF_32,
  106|       |        Self::UTF_32LE,
  107|       |        Self::UTF_32BE,
  108|       |    ];
  109|       |
  110|       |    /// Creates an unchecked static charset descriptor.
  111|       |    ///
  112|       |    /// This constructor is intended for constant initialization of built-in or
  113|       |    /// otherwise statically audited charset metadata. It performs no
  114|       |    /// validation: identifiers, display names, and aliases may be empty,
  115|       |    /// duplicate existing labels, or normalize to unusable lookup keys. The
  116|       |    /// caller must ensure every supplied label is valid for the intended use.
  117|       |    ///
  118|       |    /// Use [`Self::try_new`] when descriptor labels should be validated before
  119|       |    /// a value is used at runtime, or [`Self::register_new`] when the
  120|       |    /// descriptor should also be inserted into the runtime registry.
  121|       |    ///
  122|       |    /// # Parameters
  123|       |    ///
  124|       |    /// - `id`: Stable normalized identifier used for equality and hashing.
  125|       |    /// - `name`: Human-readable display name.
  126|       |    /// - `aliases`: Additional labels accepted for this charset.
  127|       |    ///
  128|       |    /// # Returns
  129|       |    ///
  130|       |    /// Returns a charset descriptor carrying the supplied metadata.
  131|       |    #[inline]
  132|     25|    pub const fn new_static(id: &'static str, name: &'static str, aliases: &'static [&'static str]) -> Self {
  133|     25|        Self { id, name, aliases }
  134|     25|    }
  135|       |
  136|       |    /// Creates a validated static charset descriptor without registering it.
  137|       |    ///
  138|       |    /// Validation uses the same labels and loose conflict checks as
  139|       |    /// [`Self::register`], but does not mutate the global runtime registry.
  140|       |    ///
  141|       |    /// # Parameters
  142|       |    ///
  143|       |    /// - `id`: Stable normalized identifier used for equality and hashing.
  144|       |    /// - `name`: Human-readable display name.
  145|       |    /// - `aliases`: Additional labels accepted for this charset.
  146|       |    ///
  147|       |    /// # Returns
  148|       |    ///
  149|       |    /// Returns a charset descriptor when every descriptor label is usable and
  150|       |    /// does not conflict with a different built-in or registered charset.
  151|       |    ///
  152|       |    /// # Errors
  153|       |    ///
  154|       |    /// Returns [`CharsetRegistrationError`] when any candidate label normalizes
  155|       |    /// to an empty lookup key or conflicts with an existing charset.
  156|      7|    pub fn try_new(
  157|      7|        id: &'static str,
  158|      7|        name: &'static str,
  159|      7|        aliases: &'static [&'static str],
  160|      7|    ) -> Result<Self, CharsetRegistrationError> {
  161|      7|        let candidate = Self::new_static(id, name, aliases);
  162|      7|        validate_descriptor(candidate, &read_registry())
  163|      7|    }
  164|       |
  165|       |    /// Creates and registers a charset descriptor.
  166|       |    ///
  167|       |    /// # Parameters
  168|       |    ///
  169|       |    /// - `id`: Stable normalized identifier used for equality and hashing.
  170|       |    /// - `name`: Human-readable display name.
  171|       |    /// - `aliases`: Additional labels accepted for this charset.
  172|       |    ///
  173|       |    /// # Returns
  174|       |    ///
  175|       |    /// Returns the registered descriptor when no label conflicts exist.
  176|       |    ///
  177|       |    /// # Errors
  178|       |    ///
  179|       |    /// Returns [`CharsetRegistrationError`] when the descriptor's identifier,
  180|       |    /// display name, or aliases normalize to an empty label or conflict with a
  181|       |    /// built-in or previously registered charset under loose label
  182|       |    /// normalization.
  183|      4|    pub fn register_new(
  184|      4|        id: &'static str,
  185|      4|        name: &'static str,
  186|      4|        aliases: &'static [&'static str],
  187|      4|    ) -> Result<Self, CharsetRegistrationError> {
  188|      4|        Self::try_new(id, name, aliases)?.register()
                                                      ^1^3
  189|      4|    }
  190|       |
  191|       |    /// Registers this descriptor in the global charset registry.
  192|       |    ///
  193|       |    /// # Returns
  194|       |    ///
  195|       |    /// Returns this descriptor when registration succeeds. Registering the
  196|       |    /// exact same descriptor more than once is idempotent and returns the
  197|       |    /// already known descriptor. After registration, [`Self::from_label`] and
  198|       |    /// [`Self::from_whatwg_label`] can discover the charset by identifier,
  199|       |    /// display name, or alias.
  200|       |    ///
  201|       |    /// # Errors
  202|       |    ///
  203|       |    /// Returns [`CharsetRegistrationError`] when any candidate label normalizes
  204|       |    /// to an empty label or conflicts with a different built-in or registered
  205|       |    /// charset under loose label normalization.
  206|     13|    pub fn register(self) -> Result<Self, CharsetRegistrationError> {
  207|     13|        let mut registry = write_registry();
  208|     13|        let descriptor = validate_descriptor(self, &registry)?;
                          ^9                                               ^4
  209|      9|        if descriptor_exists(descriptor, &registry) {
  210|      1|            return Ok(descriptor);
  211|      8|        }
  212|      8|        registry.push(descriptor);
  213|      8|        Ok(descriptor)
  214|     13|    }
  215|       |
  216|       |    /// Returns a snapshot of runtime-registered charsets.
  217|       |    ///
  218|       |    /// # Returns
  219|       |    ///
  220|       |    /// Returns the charsets inserted through [`Self::register`] or
  221|       |    /// [`Self::register_new`]. Built-in charsets are available through
  222|       |    /// [`Self::BUILTINS`] and are not included in this snapshot.
  223|       |    ///
  224|       |    /// # Panics
  225|       |    ///
  226|       |    /// Panics if the global registry lock is poisoned.
  227|       |    #[must_use]
  228|      1|    pub fn registered() -> Vec<Self> {
  229|      1|        read_registry().clone()
  230|      1|    }
  231|       |
  232|       |    /// Finds a built-in or registered charset by loose label matching.
  233|       |    ///
  234|       |    /// # Parameters
  235|       |    ///
  236|       |    /// - `label`: Charset label to match against identifiers, names, and
  237|       |    ///   aliases.
  238|       |    ///
  239|       |    /// # Returns
  240|       |    ///
  241|       |    /// Returns `Some(Charset)` when `label` names a built-in or registered
  242|       |    /// charset, or `None` when no charset matches. The label is normalized with
  243|       |    /// [`crate::normalize_label_loose`], which trims ASCII whitespace, folds
  244|       |    /// ASCII case, and ignores `-` / `_` separators.
  245|     23|    pub fn from_label(label: &str) -> Option<Self> {
  246|     23|        Self::from_normalized_label(&normalize_label_loose(label), LabelNormalization::Loose)
  247|     23|    }
  248|       |
  249|       |    /// Finds a built-in or registered charset by WHATWG-style label matching.
  250|       |    ///
  251|       |    /// # Parameters
  252|       |    ///
  253|       |    /// - `label`: Charset label to match against identifiers, names, and
  254|       |    ///   aliases.
  255|       |    ///
  256|       |    /// # Returns
  257|       |    ///
  258|       |    /// Returns `Some(Charset)` when `label` names a built-in or registered
  259|       |    /// charset, or `None` when no charset matches. The label is normalized with
  260|       |    /// [`crate::normalize_label_whatwg`], which trims ASCII whitespace and
  261|       |    /// folds ASCII case while preserving punctuation and separators.
  262|       |    ///
  263|       |    /// This applies WHATWG-style preprocessing only to this crate's own
  264|       |    /// charset descriptor table. It is not the full WHATWG Encoding Standard
  265|       |    /// label table, and it does not remap charset semantics such as treating
  266|       |    /// `iso-8859-1` as Windows-1252.
  267|      8|    pub fn from_whatwg_label(label: &str) -> Option<Self> {
  268|      8|        Self::from_normalized_label(&normalize_label_whatwg(label), LabelNormalization::Whatwg)
  269|      8|    }
  270|       |
  271|       |    /// Returns the stable normalized charset identifier.
  272|       |    ///
  273|       |    /// # Returns
  274|       |    ///
  275|       |    /// Returns the identifier used for equality, hashing, and stable
  276|       |    /// diagnostic output.
  277|       |    #[inline]
  278|      8|    pub const fn id(self) -> &'static str {
  279|      8|        self.id
  280|      8|    }
  281|       |
  282|       |    /// Returns a human-readable charset label.
  283|       |    ///
  284|       |    /// # Returns
  285|       |    ///
  286|       |    /// Returns the display name stored in this descriptor.
  287|       |    #[inline]
  288|     23|    pub const fn name(self) -> &'static str {
  289|     23|        self.name
  290|     23|    }
  291|       |
  292|       |    /// Returns accepted aliases for this charset.
  293|       |    ///
  294|       |    /// # Returns
  295|       |    ///
  296|       |    /// Returns the static alias list stored in this descriptor.
  297|       |    #[inline]
  298|      3|    pub const fn aliases(self) -> &'static [&'static str] {
  299|      3|        self.aliases
  300|      3|    }
  301|       |
  302|       |    /// Returns the UTF-16 charset with a fixed byte order.
  303|       |    ///
  304|       |    /// # Parameters
  305|       |    ///
  306|       |    /// - `byte_order`: The byte order used by the byte stream.
  307|       |    ///
  308|       |    /// # Returns
  309|       |    ///
  310|       |    /// Returns [`Self::UTF_16LE`] for little-endian byte order and
  311|       |    /// [`Self::UTF_16BE`] for big-endian byte order.
  312|       |    #[inline]
  313|  16.3k|    pub const fn from_utf16_byte_order(byte_order: ByteOrder) -> Self {
  314|      0|        match byte_order {
  315|  16.2k|            ByteOrder::LittleEndian => Self::UTF_16LE,
  316|    136|            ByteOrder::BigEndian => Self::UTF_16BE,
  317|      0|            ByteOrder::NativeEndian if cfg!(target_endian = "little") => Self::UTF_16LE,
  318|      0|            ByteOrder::NativeEndian => Self::UTF_16BE,
  319|       |        }
  320|  16.3k|    }
  321|       |
  322|       |    /// Returns the UTF-32 charset with a fixed byte order.
  323|       |    ///
  324|       |    /// # Parameters
  325|       |    ///
  326|       |    /// - `byte_order`: The byte order used by the byte stream.
  327|       |    ///
  328|       |    /// # Returns
  329|       |    ///
  330|       |    /// Returns [`Self::UTF_32LE`] for little-endian byte order and
  331|       |    /// [`Self::UTF_32BE`] for big-endian byte order.
  332|       |    #[inline]
  333|    264|    pub const fn from_utf32_byte_order(byte_order: ByteOrder) -> Self {
  334|      0|        match byte_order {
  335|    133|            ByteOrder::LittleEndian => Self::UTF_32LE,
  336|    131|            ByteOrder::BigEndian => Self::UTF_32BE,
  337|      0|            ByteOrder::NativeEndian if cfg!(target_endian = "little") => Self::UTF_32LE,
  338|      0|            ByteOrder::NativeEndian => Self::UTF_32BE,
  339|       |        }
  340|    264|    }
  341|       |
  342|       |    /// Returns the fixed byte order represented by this charset.
  343|       |    ///
  344|       |    /// # Returns
  345|       |    ///
  346|       |    /// Returns `Some(ByteOrder)` for fixed-endian UTF-16 and UTF-32 charsets.
  347|       |    /// Returns `None` for UTF-8 and generic UTF-16/UTF-32 charsets.
  348|       |    #[inline]
  349|      4|    pub fn byte_order(self) -> Option<ByteOrder> {
  350|      4|        match self.id {
  351|      4|            "utf-16le" | "utf-32le" => Some(ByteOrder::LittleEndian),
                                       ^3            ^1
  352|      3|            "utf-16be" | "utf-32be" => Some(ByteOrder::BigEndian),
                                                     ^1
  353|      2|            _ => None,
  354|       |        }
  355|      4|    }
  356|       |
  357|       |    /// Tests whether a label names this charset.
  358|       |    ///
  359|       |    /// # Parameters
  360|       |    ///
  361|       |    /// - `label`: The label to compare with this descriptor's identifier,
  362|       |    ///   display name, and aliases.
  363|       |    ///
  364|       |    /// # Returns
  365|       |    ///
  366|       |    /// Returns `true` when `label` matches the identifier, display name, or one
  367|       |    /// of the aliases using loose label normalization.
  368|     17|    pub fn matches_label(self, label: &str) -> bool {
  369|     17|        let label = normalize_label_loose(label);
  370|     17|        self.matches_normalized_label(&label, LabelNormalization::Loose)
  371|     17|    }
  372|       |
  373|       |    /// Finds a charset by a pre-normalized label.
  374|       |    ///
  375|       |    /// # Parameters
  376|       |    ///
  377|       |    /// - `label`: Normalized label to search for.
  378|       |    /// - `normalize`: Normalization function used for charset-owned labels.
  379|       |    ///
  380|       |    /// # Returns
  381|       |    ///
  382|       |    /// Returns the first built-in or registered charset whose labels normalize
  383|       |    /// to `label`, or `None` when no charset matches.
  384|       |    #[inline]
  385|     31|    fn from_normalized_label(label: &str, normalization: LabelNormalization) -> Option<Self> {
  386|     31|        if label.is_empty() {
  387|      1|            return None;
  388|     30|        }
  389|     30|        Self::BUILTINS
  390|     30|            .iter()
  391|     30|            .copied()
  392|    195|            .find(|charset| charset.matches_normalized_label(label, normalization))
                           ^30
  393|     30|            .or_else(|| {
                                      ^15
  394|     15|                let registry = read_registry();
  395|     15|                registry
  396|     15|                    .iter()
  397|     15|                    .copied()
  398|     39|                    .find(|charset| charset.matches_normalized_label(label, normalization))
                                   ^15
  399|     15|            })
  400|     31|    }
  401|       |
  402|       |    /// Tests this descriptor against a pre-normalized label.
  403|       |    ///
  404|       |    /// # Parameters
  405|       |    ///
  406|       |    /// - `label`: Normalized caller-provided label.
  407|       |    /// - `normalize`: Normalization function applied to this descriptor's
  408|       |    ///   labels.
  409|       |    ///
  410|       |    /// # Returns
  411|       |    ///
  412|       |    /// Returns `true` when any descriptor label normalizes to `label`.
  413|       |    #[inline]
  414|    784|    fn matches_normalized_label(self, label: &str, normalization: LabelNormalization) -> bool {
  415|  3.04k|        charset_labels(self).any(|candidate| label_matches_normalized(candidate, label, normalization))
                      ^784                 ^784
  416|    784|    }
  417|       |}
  418|       |
  419|       |impl PartialEq for Charset {
  420|       |    /// Compares charsets by stable identifier.
  421|       |    ///
  422|       |    /// # Parameters
  423|       |    ///
  424|       |    /// - `other`: The descriptor to compare against.
  425|       |    ///
  426|       |    /// # Returns
  427|       |    ///
  428|       |    /// Returns `true` when both descriptors have the same identifier.
  429|       |    #[inline]
  430|    112|    fn eq(&self, other: &Self) -> bool {
  431|    112|        self.id == other.id
  432|    112|    }
  433|       |}
  434|       |
  435|       |impl Eq for Charset {}
  436|       |
  437|       |impl Hash for Charset {
  438|       |    /// Hashes the stable encoding identifier.
  439|       |    ///
  440|       |    /// # Parameters
  441|       |    ///
  442|       |    /// - `state`: The hasher receiving this encoding's identity.
  443|       |    #[inline]
  444|      2|    fn hash<H>(&self, state: &mut H)
  445|      2|    where
  446|      2|        H: Hasher,
  447|       |    {
  448|      2|        self.id.hash(state);
  449|      2|    }
  450|       |}
  451|       |
  452|       |impl fmt::Display for Charset {
  453|       |    /// Formats this charset label.
  454|       |    ///
  455|       |    /// The display form is the human-readable [`Charset::name`]. Use
  456|       |    /// [`Charset::id`] for stable diagnostics or cross-language comparison.
  457|       |    ///
  458|       |    /// # Parameters
  459|       |    ///
  460|       |    /// - `formatter`: The formatter receiving the label.
  461|       |    ///
  462|       |    /// # Errors
  463|       |    ///
  464|       |    /// Returns any formatting error reported by `formatter`.
  465|       |    #[inline]
  466|     15|    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
  467|     15|        formatter.write_str(self.name())
  468|     15|    }
  469|       |}
  470|       |
  471|       |#[cfg(feature = "serde")]
  472|       |impl Serialize for Charset {
  473|       |    /// Serializes this charset as its stable identifier.
  474|       |    ///
  475|       |    /// # Parameters
  476|       |    ///
  477|       |    /// - `serializer`: Serde serializer receiving the identifier string.
  478|       |    ///
  479|       |    /// # Errors
  480|       |    ///
  481|       |    /// Returns any serialization error reported by `serializer`.
  482|       |    #[inline]
  483|      1|    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  484|      1|    where
  485|      1|        S: Serializer,
  486|       |    {
  487|      1|        serializer.serialize_str(self.id())
  488|      1|    }
  489|       |}
  490|       |
  491|       |#[cfg(feature = "serde")]
  492|       |impl<'de> Deserialize<'de> for Charset {
  493|       |    /// Deserializes a charset from its stable identifier.
  494|       |    ///
  495|       |    /// The identifier must already name a built-in or runtime-registered
  496|       |    /// charset in this process.
  497|       |    ///
  498|       |    /// # Parameters
  499|       |    ///
  500|       |    /// - `deserializer`: Serde deserializer providing the identifier string.
  501|       |    ///
  502|       |    /// # Errors
  503|       |    ///
  504|       |    /// Returns a serde error when the string is not a known charset label.
  505|      4|    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  506|      4|    where
  507|      4|        D: Deserializer<'de>,
  508|       |    {
  509|      4|        let id = <&str>::deserialize(deserializer)?;
                          ^3                                    ^1
  510|      3|        Self::from_label(id).ok_or_else(|| de::Error::custom(format!("unknown charset `{id}`")))
                                                         ^1                ^1
  511|      4|    }
  512|       |}
  513|       |
  514|       |/// Returns the global charset registry.
  515|       |///
  516|       |/// # Returns
  517|       |///
  518|       |/// Returns the lazily initialized registry used by [`Charset::register`].
  519|       |#[inline]
  520|     36|fn registry() -> &'static RwLock<Vec<Charset>> {
  521|     36|    CHARSET_REGISTRY.get_or_init(|| RwLock::new(Vec::new()))
                                                  ^1          ^1
  522|     36|}
  523|       |
  524|       |/// Reads the global charset registry.
  525|       |///
  526|       |/// # Returns
  527|       |///
  528|       |/// Returns a read guard for the registry.
  529|       |///
  530|       |/// # Panics
  531|       |///
  532|       |/// Panics if the registry lock is poisoned.
  533|       |#[inline]
  534|     23|fn read_registry() -> RwLockReadGuard<'static, Vec<Charset>> {
  535|     23|    registry().read().expect("charset registry lock should not be poisoned")
  536|     23|}
  537|       |
  538|       |/// Writes the global charset registry.
  539|       |///
  540|       |/// # Returns
  541|       |///
  542|       |/// Returns a write guard for the registry.
  543|       |///
  544|       |/// # Panics
  545|       |///
  546|       |/// Panics if the registry lock is poisoned.
  547|       |#[inline]
  548|     13|fn write_registry() -> RwLockWriteGuard<'static, Vec<Charset>> {
  549|     13|    registry()
  550|     13|        .write()
  551|     13|        .expect("charset registry lock should not be poisoned")
  552|     13|}
  553|       |
  554|       |/// Iterates over every label owned by `charset`.
  555|       |///
  556|       |/// # Parameters
  557|       |///
  558|       |/// - `charset`: Charset descriptor whose labels are inspected.
  559|       |///
  560|       |/// # Returns
  561|       |///
  562|       |/// Returns an iterator over the identifier, display name, and aliases.
  563|       |#[inline]
  564|    819|fn charset_labels(charset: Charset) -> impl Iterator<Item = &'static str> {
  565|    819|    std::iter::once(charset.id)
  566|    819|        .chain(std::iter::once(charset.name))
  567|    819|        .chain(charset.aliases.iter().copied())
  568|    819|}
  569|       |
  570|       |/// Finds the first invalid label in `candidate`.
  571|       |///
  572|       |/// # Parameters
  573|       |///
  574|       |/// - `candidate`: Charset descriptor being validated.
  575|       |///
  576|       |/// # Returns
  577|       |///
  578|       |/// Returns the first descriptor label whose loose-normalized form is empty, or
  579|       |/// `None` when all labels can be used for lookup.
  580|       |#[inline]
  581|     20|fn invalid_label_for(candidate: Charset) -> Option<&'static str> {
  582|     52|    charset_labels(candidate).find(|label| normalize_label_loose(label).is_empty())
                  ^20                       ^20
  583|     20|}
  584|       |
  585|       |/// Validates a descriptor against built-ins and a registry snapshot.
  586|       |///
  587|       |/// # Parameters
  588|       |///
  589|       |/// - `candidate`: Charset descriptor to validate.
  590|       |/// - `registered`: Runtime registry contents to compare against.
  591|       |///
  592|       |/// # Returns
  593|       |///
  594|       |/// Returns `candidate` when all labels are valid and unambiguous. If the only
  595|       |/// conflict is an identical existing descriptor, returns that existing
  596|       |/// descriptor so repeated registration stays idempotent.
  597|     20|fn validate_descriptor(candidate: Charset, registered: &[Charset]) -> Result<Charset, CharsetRegistrationError> {
  598|     20|    if let Some(label) = invalid_label_for(candidate) {
                              ^5
  599|      5|        return Err(CharsetRegistrationError::invalid_label(label, candidate));
  600|     15|    }
  601|     15|    if let Some((label, existing)) = conflict_for(candidate, registered) {
                               ^3     ^3
  602|      3|        if same_descriptor(existing, candidate) {
  603|      1|            return Ok(existing);
  604|      2|        }
  605|      2|        return Err(CharsetRegistrationError::conflicting_label(label, existing, candidate));
  606|     12|    }
  607|     12|    Ok(candidate)
  608|     20|}
  609|       |
  610|       |/// Tests whether a descriptor already exists in built-ins or the registry.
  611|       |///
  612|       |/// # Parameters
  613|       |///
  614|       |/// - `candidate`: Charset descriptor to search for.
  615|       |/// - `registered`: Runtime registry contents to compare against.
  616|       |///
  617|       |/// # Returns
  618|       |///
  619|       |/// Returns `true` when an identical descriptor is already known.
  620|       |#[inline]
  621|      9|fn descriptor_exists(candidate: Charset, registered: &[Charset]) -> bool {
  622|      9|    Charset::BUILTINS
  623|      9|        .iter()
  624|      9|        .copied()
  625|      9|        .chain(registered.iter().copied())
  626|    111|        .any(|existing| same_descriptor(existing, candidate))
                       ^9
  627|      9|}
  628|       |
  629|       |/// Tests whether two descriptors carry exactly the same public metadata.
  630|       |///
  631|       |/// # Parameters
  632|       |///
  633|       |/// - `left`: First descriptor.
  634|       |/// - `right`: Second descriptor.
  635|       |///
  636|       |/// # Returns
  637|       |///
  638|       |/// Returns `true` when id, display name, and alias list are identical.
  639|       |#[inline]
  640|    114|fn same_descriptor(left: Charset, right: Charset) -> bool {
  641|    114|    left.id == right.id && left.name == right.name && left.aliases == right.aliases
                                         ^2                         ^2
  642|    114|}
  643|       |
  644|       |/// Finds a label conflict for `candidate`.
  645|       |///
  646|       |/// # Parameters
  647|       |///
  648|       |/// - `candidate`: Charset descriptor being registered.
  649|       |/// - `registered`: Runtime registry contents to compare against.
  650|       |///
  651|       |/// # Returns
  652|       |///
  653|       |/// Returns the conflicting candidate label and existing charset, or `None`
  654|       |/// when all candidate labels are available.
  655|     15|fn conflict_for(candidate: Charset, registered: &[Charset]) -> Option<(&'static str, Charset)> {
  656|     43|    for label in charset_labels(candidate) {
                               ^15            ^15
  657|     43|        let normalized = normalize_label_loose(label);
  658|     43|        if let Some(existing) = Charset::BUILTINS
                                  ^3
  659|     43|            .iter()
  660|     43|            .copied()
  661|     43|            .chain(registered.iter().copied())
  662|    533|            .find(|existing| existing.matches_normalized_label(&normalized, LabelNormalization::Loose))
                           ^43
  663|       |        {
  664|      3|            return Some((label, existing));
  665|     40|        }
  666|       |    }
  667|     12|    None
  668|     15|}
  669|       |
  670|       |/// Tests whether `candidate` equals a pre-normalized label.
  671|       |///
  672|       |/// # Parameters
  673|       |///
  674|       |/// - `candidate`: Charset-owned label to normalize lazily.
  675|       |/// - `normalized`: Caller-provided normalized label.
  676|       |/// - `normalization`: Normalization semantics to apply.
  677|       |///
  678|       |/// # Returns
  679|       |///
  680|       |/// Returns `true` when `candidate` normalizes to `normalized`.
  681|       |#[inline]
  682|  3.04k|fn label_matches_normalized(candidate: &str, normalized: &str, normalization: LabelNormalization) -> bool {
  683|  3.04k|    match normalization {
  684|  2.77k|        LabelNormalization::Loose => loose_label_matches_normalized(candidate, normalized),
  685|    269|        LabelNormalization::Whatwg => whatwg_label_matches_normalized(candidate, normalized),
  686|       |    }
  687|  3.04k|}
  688|       |
  689|       |/// Tests loose-normalized equality without allocating a temporary `String`.
  690|       |#[inline]
  691|  2.77k|fn loose_label_matches_normalized(candidate: &str, normalized: &str) -> bool {
  692|  2.77k|    let mut expected = normalized.chars();
  693|  6.23k|    for ch in candidate.trim_ascii().chars() {
                            ^2.77k                 ^2.77k
  694|  6.23k|        if matches!(ch, '-' | '_') {
                         ^5.60k
  695|    628|            continue;
  696|  5.60k|        }
  697|  5.60k|        match expected.next() {
  698|  5.60k|            Some(expected_ch) if expected_ch == ch.to_ascii_lowercase() => {}
                               ^2.89k                                               ^2.89k^2.89k
  699|  2.71k|            _ => return false,
  700|       |        }
  701|       |    }
  702|     61|    expected.next().is_none()
  703|  2.77k|}
  704|       |
  705|       |/// Tests WHATWG-preprocessed equality without allocating a temporary `String`.
  706|       |#[inline]
  707|    269|fn whatwg_label_matches_normalized(candidate: &str, normalized: &str) -> bool {
  708|    269|    let mut expected = normalized.chars();
  709|    814|    for ch in candidate.trim_ascii().chars() {
                            ^269                   ^269
  710|    814|        match expected.next() {
  711|    814|            Some(expected_ch) if expected_ch == ch.to_ascii_lowercase() => {}
                               ^557                                                 ^557 ^557
  712|    257|            _ => return false,
  713|       |        }
  714|       |    }
  715|     12|    expected.next().is_none()
  716|    269|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/charset_registration_error.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::error::Error;
    9|       |use core::fmt;
   10|       |
   11|       |use super::charset::Charset;
   12|       |use super::charset_registration_error_kind::CharsetRegistrationErrorKind;
   13|       |
   14|       |/// Error returned when a charset cannot be registered.
   15|       |///
   16|       |/// Registration rejects labels that normalize to an empty lookup key or would
   17|       |/// make global lookup ambiguous. Label comparison uses the same loose
   18|       |/// normalization as [`Charset::from_label`], so conflicts include case
   19|       |/// differences and `-` / `_` separator differences.
   20|       |#[derive(Clone, Copy, Debug, Eq, PartialEq)]
   21|       |pub struct CharsetRegistrationError {
   22|       |    /// Candidate label that failed validation or conflicts.
   23|       |    label: &'static str,
   24|       |    /// Registration failure category.
   25|       |    kind: CharsetRegistrationErrorKind,
   26|       |    /// Charset that failed to register.
   27|       |    candidate: Charset,
   28|       |}
   29|       |
   30|       |impl CharsetRegistrationError {
   31|       |    /// Creates a conflicting-label registration error.
   32|       |    ///
   33|       |    /// # Parameters
   34|       |    ///
   35|       |    /// - `label`: Candidate label that conflicts.
   36|       |    /// - `existing`: Charset already owning the label.
   37|       |    /// - `candidate`: Charset being registered.
   38|       |    ///
   39|       |    /// # Returns
   40|       |    ///
   41|       |    /// Returns an error carrying the conflicting registration context.
   42|       |    #[inline]
   43|      2|    pub(crate) const fn conflicting_label(label: &'static str, existing: Charset, candidate: Charset) -> Self {
   44|      2|        Self {
   45|      2|            label,
   46|      2|            kind: CharsetRegistrationErrorKind::ConflictingLabel { existing },
   47|      2|            candidate,
   48|      2|        }
   49|      2|    }
   50|       |
   51|       |    /// Creates an invalid-label registration error.
   52|       |    ///
   53|       |    /// # Parameters
   54|       |    ///
   55|       |    /// - `label`: Candidate label that is invalid.
   56|       |    /// - `candidate`: Charset being registered.
   57|       |    ///
   58|       |    /// # Returns
   59|       |    ///
   60|       |    /// Returns an error carrying the invalid registration context.
   61|       |    #[inline]
   62|      5|    pub(crate) const fn invalid_label(label: &'static str, candidate: Charset) -> Self {
   63|      5|        Self {
   64|      5|            label,
   65|      5|            kind: CharsetRegistrationErrorKind::InvalidLabel,
   66|      5|            candidate,
   67|      5|        }
   68|      5|    }
   69|       |
   70|       |    /// Returns the candidate label that caused the error.
   71|       |    ///
   72|       |    /// # Returns
   73|       |    ///
   74|       |    /// Returns the static label from the candidate charset descriptor.
   75|       |    #[inline]
   76|      6|    pub const fn label(self) -> &'static str {
   77|      6|        self.label
   78|      6|    }
   79|       |
   80|       |    /// Returns the registration failure category.
   81|       |    ///
   82|       |    /// # Returns
   83|       |    ///
   84|       |    /// Returns the stored [`CharsetRegistrationErrorKind`].
   85|       |    #[inline]
   86|      7|    pub const fn kind(self) -> CharsetRegistrationErrorKind {
   87|      7|        self.kind
   88|      7|    }
   89|       |
   90|       |    /// Returns the charset that already owns the conflicting label.
   91|       |    ///
   92|       |    /// # Returns
   93|       |    ///
   94|       |    /// Returns `Some(charset)` for conflicting labels, or `None` when the
   95|       |    /// candidate label itself is invalid.
   96|       |    #[inline]
   97|      4|    pub const fn existing(self) -> Option<Charset> {
   98|      4|        match self.kind {
   99|      1|            CharsetRegistrationErrorKind::ConflictingLabel { existing } => Some(existing),
  100|      3|            CharsetRegistrationErrorKind::InvalidLabel => None,
  101|       |        }
  102|      4|    }
  103|       |
  104|       |    /// Returns the charset that failed to register.
  105|       |    ///
  106|       |    /// # Returns
  107|       |    ///
  108|       |    /// Returns the candidate charset descriptor.
  109|       |    #[inline]
  110|      3|    pub const fn candidate(self) -> Charset {
  111|      3|        self.candidate
  112|      3|    }
  113|       |}
  114|       |
  115|       |impl fmt::Display for CharsetRegistrationError {
  116|       |    /// Formats this registration error.
  117|       |    ///
  118|       |    /// # Parameters
  119|       |    ///
  120|       |    /// - `formatter`: The formatter receiving the diagnostic message.
  121|       |    ///
  122|       |    /// # Errors
  123|       |    ///
  124|       |    /// Returns any formatting error reported by `formatter`.
  125|      3|    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
  126|      3|        match self.kind {
  127|       |            CharsetRegistrationErrorKind::InvalidLabel => {
  128|      2|                write!(
  129|      2|                    formatter,
  130|       |                    "charset label {:?} for {} is invalid",
  131|       |                    self.label, self.candidate,
  132|       |                )
  133|       |            }
  134|      1|            CharsetRegistrationErrorKind::ConflictingLabel { existing } => {
  135|      1|                write!(
  136|      1|                    formatter,
  137|       |                    "charset label {:?} for {} conflicts with {}",
  138|       |                    self.label, self.candidate, existing,
  139|       |                )
  140|       |            }
  141|       |        }
  142|      3|    }
  143|       |}
  144|       |
  145|       |impl Error for CharsetRegistrationError {}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/latin1.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use super::Ascii;
    9|       |
   10|       |/// Namespace for ISO-8859-1 / Latin-1 character and code point helpers.
   11|       |pub enum Latin1 {}
   12|       |
   13|       |impl Latin1 {
   14|       |    /// Maximum valid Latin-1 character.
   15|       |    pub const MAX_CHAR: char = '\u{00ff}';
   16|       |
   17|       |    /// Maximum valid Latin-1 byte.
   18|       |    pub const MAX_BYTE: u8 = 0xff;
   19|       |
   20|       |    /// Maximum valid Latin-1 code point.
   21|       |    pub const MAX_CODE_POINT: u32 = Self::MAX_BYTE as u32;
   22|       |
   23|       |    /// Tests whether a byte is a Latin-1 byte.
   24|       |    ///
   25|       |    /// # Parameters
   26|       |    ///
   27|       |    /// - `_value`: The byte to test.
   28|       |    ///
   29|       |    /// # Returns
   30|       |    ///
   31|       |    /// Returns `true` for every byte value because ISO-8859-1 maps all
   32|       |    /// `0x00..=0xFF` bytes.
   33|       |    #[inline(always)]
   34|       |    #[must_use]
   35|      1|    pub const fn is_latin1_byte(_value: u8) -> bool {
   36|      1|        true
   37|      1|    }
   38|       |
   39|       |    /// Tests whether a character is a Latin-1 character.
   40|       |    ///
   41|       |    /// # Parameters
   42|       |    ///
   43|       |    /// - `value`: The character to test.
   44|       |    ///
   45|       |    /// # Returns
   46|       |    ///
   47|       |    /// Returns `true` if `value` is in `U+0000..=U+00FF`.
   48|       |    #[inline]
   49|       |    #[must_use]
   50|     51|    pub const fn is_latin1_char(value: char) -> bool {
   51|     51|        value <= Self::MAX_CHAR
   52|     51|    }
   53|       |
   54|       |    /// Tests whether an integer value is a Latin-1 code point.
   55|       |    ///
   56|       |    /// # Parameters
   57|       |    ///
   58|       |    /// - `value`: The raw code point value to test.
   59|       |    ///
   60|       |    /// # Returns
   61|       |    ///
   62|       |    /// Returns `true` if `value` is in `0x00..=0xFF`.
   63|       |    #[inline]
   64|       |    #[must_use]
   65|     38|    pub const fn is_latin1_code_point(value: u32) -> bool {
   66|     38|        value <= Self::MAX_CODE_POINT
   67|     38|    }
   68|       |
   69|       |    /// Tests whether a byte is Java-style Latin-1 whitespace.
   70|       |    ///
   71|       |    /// # Parameters
   72|       |    ///
   73|       |    /// - `value`: The byte to test.
   74|       |    ///
   75|       |    /// # Returns
   76|       |    ///
   77|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
   78|       |    #[inline(always)]
   79|       |    #[must_use]
   80|      4|    pub const fn is_whitespace_byte(value: u8) -> bool {
   81|      4|        Ascii::is_whitespace_byte(value)
   82|      4|    }
   83|       |
   84|       |    /// Tests whether a character is Java-style Latin-1 whitespace.
   85|       |    ///
   86|       |    /// # Parameters
   87|       |    ///
   88|       |    /// - `value`: The character to test.
   89|       |    ///
   90|       |    /// # Returns
   91|       |    ///
   92|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
   93|       |    #[inline]
   94|       |    #[must_use]
   95|      3|    pub const fn is_whitespace_char(value: char) -> bool {
   96|      3|        match Self::char_to_byte(value) {
   97|      2|            Some(value) => Self::is_whitespace_byte(value),
   98|      1|            None => false,
   99|       |        }
  100|      3|    }
  101|       |
  102|       |    /// Tests whether a raw code point is Java-style Latin-1 whitespace.
  103|       |    ///
  104|       |    /// # Parameters
  105|       |    ///
  106|       |    /// - `value`: The raw code point value to test.
  107|       |    ///
  108|       |    /// # Returns
  109|       |    ///
  110|       |    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
  111|       |    #[inline]
  112|       |    #[must_use]
  113|      2|    pub const fn is_whitespace_code_point(value: u32) -> bool {
  114|      2|        match Self::code_point_to_byte(value) {
  115|      1|            Some(value) => Self::is_whitespace_byte(value),
  116|      1|            None => false,
  117|       |        }
  118|      2|    }
  119|       |
  120|       |    /// Tests whether a byte is a Latin-1 letter.
  121|       |    ///
  122|       |    /// # Parameters
  123|       |    ///
  124|       |    /// - `value`: The byte to test.
  125|       |    ///
  126|       |    /// # Returns
  127|       |    ///
  128|       |    /// Returns `true` if `value` is an ISO-8859-1 encoded letter.
  129|       |    #[inline(always)]
  130|       |    #[must_use]
  131|      9|    pub const fn is_letter_byte(value: u8) -> bool {
  132|      9|        Self::is_uppercase_letter_byte(value) || Self::is_lowercase_letter_byte(value) || value == 0xaa || value == 0xba
                                                               ^7                             ^7        ^3               ^3
  133|      9|    }
  134|       |
  135|       |    /// Tests whether a character is a Latin-1 letter.
  136|       |    ///
  137|       |    /// # Parameters
  138|       |    ///
  139|       |    /// - `value`: The character to test.
  140|       |    ///
  141|       |    /// # Returns
  142|       |    ///
  143|       |    /// Returns `true` if `value` is in the Latin-1 letter set.
  144|       |    #[inline]
  145|       |    #[must_use]
  146|      3|    pub const fn is_letter_char(value: char) -> bool {
  147|      3|        match Self::char_to_byte(value) {
  148|      2|            Some(value) => Self::is_letter_byte(value),
  149|      1|            None => false,
  150|       |        }
  151|      3|    }
  152|       |
  153|       |    /// Tests whether a raw code point is a Latin-1 letter.
  154|       |    ///
  155|       |    /// # Parameters
  156|       |    ///
  157|       |    /// - `value`: The raw code point value to test.
  158|       |    ///
  159|       |    /// # Returns
  160|       |    ///
  161|       |    /// Returns `true` if `value` is in the Latin-1 letter set.
  162|       |    #[inline]
  163|       |    #[must_use]
  164|      2|    pub const fn is_letter_code_point(value: u32) -> bool {
  165|      2|        match Self::code_point_to_byte(value) {
  166|      1|            Some(value) => Self::is_letter_byte(value),
  167|      1|            None => false,
  168|       |        }
  169|      2|    }
  170|       |
  171|       |    /// Tests whether a byte is an uppercase Latin-1 letter.
  172|       |    ///
  173|       |    /// # Parameters
  174|       |    ///
  175|       |    /// - `value`: The byte to test.
  176|       |    ///
  177|       |    /// # Returns
  178|       |    ///
  179|       |    /// Returns `true` for `A..=Z`, `U+00C0..=U+00D6`, and
  180|       |    /// `U+00D8..=U+00DE`.
  181|       |    #[inline(always)]
  182|       |    #[must_use]
  183|     14|    pub const fn is_uppercase_letter_byte(value: u8) -> bool {
  184|     14|        Ascii::is_uppercase_letter_byte(value) || (value >= 0xc0 && value <= 0xd6) || (value >= 0xd8 && value <= 0xde)
                                                                 ^12              ^11                ^10              ^7
  185|     14|    }
  186|       |
  187|       |    /// Tests whether a character is an uppercase Latin-1 letter.
  188|       |    ///
  189|       |    /// # Parameters
  190|       |    ///
  191|       |    /// - `value`: The character to test.
  192|       |    ///
  193|       |    /// # Returns
  194|       |    ///
  195|       |    /// Returns `true` if `value` is an uppercase Latin-1 letter.
  196|       |    #[inline]
  197|       |    #[must_use]
  198|      2|    pub const fn is_uppercase_letter_char(value: char) -> bool {
  199|      2|        match Self::char_to_byte(value) {
  200|      1|            Some(value) => Self::is_uppercase_letter_byte(value),
  201|      1|            None => false,
  202|       |        }
  203|      2|    }
  204|       |
  205|       |    /// Tests whether a raw code point is an uppercase Latin-1 letter.
  206|       |    ///
  207|       |    /// # Parameters
  208|       |    ///
  209|       |    /// - `value`: The raw code point value to test.
  210|       |    ///
  211|       |    /// # Returns
  212|       |    ///
  213|       |    /// Returns `true` if `value` is an uppercase Latin-1 letter.
  214|       |    #[inline]
  215|       |    #[must_use]
  216|      2|    pub const fn is_uppercase_letter_code_point(value: u32) -> bool {
  217|      2|        match Self::code_point_to_byte(value) {
  218|      1|            Some(value) => Self::is_uppercase_letter_byte(value),
  219|      1|            None => false,
  220|       |        }
  221|      2|    }
  222|       |
  223|       |    /// Tests whether a byte is a lowercase Latin-1 letter.
  224|       |    ///
  225|       |    /// # Parameters
  226|       |    ///
  227|       |    /// - `value`: The byte to test.
  228|       |    ///
  229|       |    /// # Returns
  230|       |    ///
  231|       |    /// Returns `true` for `a..=z`, `U+00B5`, `U+00DF`,
  232|       |    /// `U+00E0..=U+00F6`, and `U+00F8..=U+00FF`.
  233|       |    #[inline(always)]
  234|       |    #[must_use]
  235|     13|    pub const fn is_lowercase_letter_byte(value: u8) -> bool {
  236|     13|        Ascii::is_lowercase_letter_byte(value)
  237|     13|            || value == 0xb5
  238|     12|            || value == 0xdf
  239|     11|            || (value >= 0xe0 && value <= 0xf6)
                                               ^9
  240|      7|            || value >= 0xf8
  241|     13|    }
  242|       |
  243|       |    /// Tests whether a character is a lowercase Latin-1 letter.
  244|       |    ///
  245|       |    /// # Parameters
  246|       |    ///
  247|       |    /// - `value`: The character to test.
  248|       |    ///
  249|       |    /// # Returns
  250|       |    ///
  251|       |    /// Returns `true` if `value` is a lowercase Latin-1 letter.
  252|       |    #[inline]
  253|       |    #[must_use]
  254|      2|    pub const fn is_lowercase_letter_char(value: char) -> bool {
  255|      2|        match Self::char_to_byte(value) {
  256|      1|            Some(value) => Self::is_lowercase_letter_byte(value),
  257|      1|            None => false,
  258|       |        }
  259|      2|    }
  260|       |
  261|       |    /// Tests whether a raw code point is a lowercase Latin-1 letter.
  262|       |    ///
  263|       |    /// # Parameters
  264|       |    ///
  265|       |    /// - `value`: The raw code point value to test.
  266|       |    ///
  267|       |    /// # Returns
  268|       |    ///
  269|       |    /// Returns `true` if `value` is a lowercase Latin-1 letter.
  270|       |    #[inline]
  271|       |    #[must_use]
  272|      2|    pub const fn is_lowercase_letter_code_point(value: u32) -> bool {
  273|      2|        match Self::code_point_to_byte(value) {
  274|      1|            Some(value) => Self::is_lowercase_letter_byte(value),
  275|      1|            None => false,
  276|       |        }
  277|      2|    }
  278|       |
  279|       |    /// Tests whether a byte is an ASCII decimal digit.
  280|       |    ///
  281|       |    /// # Parameters
  282|       |    ///
  283|       |    /// - `value`: The byte to test.
  284|       |    ///
  285|       |    /// # Returns
  286|       |    ///
  287|       |    /// Returns `true` for `0..=9`.
  288|       |    #[inline(always)]
  289|       |    #[must_use]
  290|      4|    pub const fn is_digit_byte(value: u8) -> bool {
  291|      4|        Ascii::is_digit_byte(value)
  292|      4|    }
  293|       |
  294|       |    /// Tests whether a character is an ASCII decimal digit in Latin-1.
  295|       |    ///
  296|       |    /// # Parameters
  297|       |    ///
  298|       |    /// - `value`: The character to test.
  299|       |    ///
  300|       |    /// # Returns
  301|       |    ///
  302|       |    /// Returns `true` for `0..=9`.
  303|       |    #[inline]
  304|       |    #[must_use]
  305|      2|    pub const fn is_digit_char(value: char) -> bool {
  306|      2|        match Self::char_to_byte(value) {
  307|      1|            Some(value) => Self::is_digit_byte(value),
  308|      1|            None => false,
  309|       |        }
  310|      2|    }
  311|       |
  312|       |    /// Tests whether a raw code point is an ASCII decimal digit in Latin-1.
  313|       |    ///
  314|       |    /// # Parameters
  315|       |    ///
  316|       |    /// - `value`: The raw code point value to test.
  317|       |    ///
  318|       |    /// # Returns
  319|       |    ///
  320|       |    /// Returns `true` for `0..=9`.
  321|       |    #[inline]
  322|       |    #[must_use]
  323|      2|    pub const fn is_digit_code_point(value: u32) -> bool {
  324|      2|        match Self::code_point_to_byte(value) {
  325|      1|            Some(value) => Self::is_digit_byte(value),
  326|      1|            None => false,
  327|       |        }
  328|      2|    }
  329|       |
  330|       |    /// Tests whether a byte is an ASCII hexadecimal digit.
  331|       |    ///
  332|       |    /// # Parameters
  333|       |    ///
  334|       |    /// - `value`: The byte to test.
  335|       |    ///
  336|       |    /// # Returns
  337|       |    ///
  338|       |    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
  339|       |    #[inline(always)]
  340|       |    #[must_use]
  341|      2|    pub const fn is_hex_digit_byte(value: u8) -> bool {
  342|      2|        Ascii::is_hex_digit_byte(value)
  343|      2|    }
  344|       |
  345|       |    /// Tests whether a character is an ASCII hexadecimal digit in Latin-1.
  346|       |    ///
  347|       |    /// # Parameters
  348|       |    ///
  349|       |    /// - `value`: The character to test.
  350|       |    ///
  351|       |    /// # Returns
  352|       |    ///
  353|       |    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
  354|       |    #[inline]
  355|       |    #[must_use]
  356|      2|    pub const fn is_hex_digit_char(value: char) -> bool {
  357|      2|        match Self::char_to_byte(value) {
  358|      1|            Some(value) => Self::is_hex_digit_byte(value),
  359|      1|            None => false,
  360|       |        }
  361|      2|    }
  362|       |
  363|       |    /// Tests whether a raw code point is an ASCII hexadecimal digit in Latin-1.
  364|       |    ///
  365|       |    /// # Parameters
  366|       |    ///
  367|       |    /// - `value`: The raw code point value to test.
  368|       |    ///
  369|       |    /// # Returns
  370|       |    ///
  371|       |    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
  372|       |    #[inline]
  373|       |    #[must_use]
  374|      2|    pub const fn is_hex_digit_code_point(value: u32) -> bool {
  375|      2|        match Self::code_point_to_byte(value) {
  376|      1|            Some(value) => Self::is_hex_digit_byte(value),
  377|      1|            None => false,
  378|       |        }
  379|      2|    }
  380|       |
  381|       |    /// Tests whether a byte is an ASCII octal digit.
  382|       |    ///
  383|       |    /// # Parameters
  384|       |    ///
  385|       |    /// - `value`: The byte to test.
  386|       |    ///
  387|       |    /// # Returns
  388|       |    ///
  389|       |    /// Returns `true` for `0..=7`.
  390|       |    #[inline(always)]
  391|       |    #[must_use]
  392|      2|    pub const fn is_octal_digit_byte(value: u8) -> bool {
  393|      2|        Ascii::is_octal_digit_byte(value)
  394|      2|    }
  395|       |
  396|       |    /// Tests whether a character is an ASCII octal digit in Latin-1.
  397|       |    ///
  398|       |    /// # Parameters
  399|       |    ///
  400|       |    /// - `value`: The character to test.
  401|       |    ///
  402|       |    /// # Returns
  403|       |    ///
  404|       |    /// Returns `true` for `0..=7`.
  405|       |    #[inline]
  406|       |    #[must_use]
  407|      2|    pub const fn is_octal_digit_char(value: char) -> bool {
  408|      2|        match Self::char_to_byte(value) {
  409|      1|            Some(value) => Self::is_octal_digit_byte(value),
  410|      1|            None => false,
  411|       |        }
  412|      2|    }
  413|       |
  414|       |    /// Tests whether a raw code point is an ASCII octal digit in Latin-1.
  415|       |    ///
  416|       |    /// # Parameters
  417|       |    ///
  418|       |    /// - `value`: The raw code point value to test.
  419|       |    ///
  420|       |    /// # Returns
  421|       |    ///
  422|       |    /// Returns `true` for `0..=7`.
  423|       |    #[inline]
  424|       |    #[must_use]
  425|      2|    pub const fn is_octal_digit_code_point(value: u32) -> bool {
  426|      2|        match Self::code_point_to_byte(value) {
  427|      1|            Some(value) => Self::is_octal_digit_byte(value),
  428|      1|            None => false,
  429|       |        }
  430|      2|    }
  431|       |
  432|       |    /// Tests whether a byte is a Latin-1 letter or ASCII decimal digit.
  433|       |    ///
  434|       |    /// # Parameters
  435|       |    ///
  436|       |    /// - `value`: The byte to test.
  437|       |    ///
  438|       |    /// # Returns
  439|       |    ///
  440|       |    /// Returns `true` for Latin-1 letters and `0..=9`.
  441|       |    #[inline(always)]
  442|       |    #[must_use]
  443|      3|    pub const fn is_letter_or_digit_byte(value: u8) -> bool {
  444|      3|        Self::is_letter_byte(value) || Self::is_digit_byte(value)
                                                     ^1                  ^1
  445|      3|    }
  446|       |
  447|       |    /// Tests whether a character is a Latin-1 letter or ASCII decimal digit.
  448|       |    ///
  449|       |    /// # Parameters
  450|       |    ///
  451|       |    /// - `value`: The character to test.
  452|       |    ///
  453|       |    /// # Returns
  454|       |    ///
  455|       |    /// Returns `true` for Latin-1 letters and `0..=9`.
  456|       |    #[inline]
  457|       |    #[must_use]
  458|      2|    pub const fn is_letter_or_digit_char(value: char) -> bool {
  459|      2|        match Self::char_to_byte(value) {
  460|      1|            Some(value) => Self::is_letter_or_digit_byte(value),
  461|      1|            None => false,
  462|       |        }
  463|      2|    }
  464|       |
  465|       |    /// Tests whether a raw code point is a Latin-1 letter or ASCII digit.
  466|       |    ///
  467|       |    /// # Parameters
  468|       |    ///
  469|       |    /// - `value`: The raw code point value to test.
  470|       |    ///
  471|       |    /// # Returns
  472|       |    ///
  473|       |    /// Returns `true` for Latin-1 letters and `0..=9`.
  474|       |    #[inline]
  475|       |    #[must_use]
  476|      2|    pub const fn is_letter_or_digit_code_point(value: u32) -> bool {
  477|      2|        match Self::code_point_to_byte(value) {
  478|      1|            Some(value) => Self::is_letter_or_digit_byte(value),
  479|      1|            None => false,
  480|       |        }
  481|      2|    }
  482|       |
  483|       |    /// Tests whether a byte is printable in Latin-1.
  484|       |    ///
  485|       |    /// # Parameters
  486|       |    ///
  487|       |    /// - `value`: The byte to test.
  488|       |    ///
  489|       |    /// # Returns
  490|       |    ///
  491|       |    /// Returns `true` for bytes outside the C0, DEL, and C1 control ranges.
  492|       |    #[inline(always)]
  493|       |    #[must_use]
  494|      5|    pub const fn is_printable_byte(value: u8) -> bool {
  495|      5|        !Self::is_control_byte(value)
  496|      5|    }
  497|       |
  498|       |    /// Tests whether a character is printable in Latin-1.
  499|       |    ///
  500|       |    /// # Parameters
  501|       |    ///
  502|       |    /// - `value`: The character to test.
  503|       |    ///
  504|       |    /// # Returns
  505|       |    ///
  506|       |    /// Returns `true` for Latin-1 characters outside control ranges.
  507|       |    #[inline]
  508|       |    #[must_use]
  509|      2|    pub const fn is_printable_char(value: char) -> bool {
  510|      2|        match Self::char_to_byte(value) {
  511|      1|            Some(value) => Self::is_printable_byte(value),
  512|      1|            None => false,
  513|       |        }
  514|      2|    }
  515|       |
  516|       |    /// Tests whether a raw code point is printable in Latin-1.
  517|       |    ///
  518|       |    /// # Parameters
  519|       |    ///
  520|       |    /// - `value`: The raw code point value to test.
  521|       |    ///
  522|       |    /// # Returns
  523|       |    ///
  524|       |    /// Returns `true` for Latin-1 values outside control ranges.
  525|       |    #[inline]
  526|       |    #[must_use]
  527|      3|    pub const fn is_printable_code_point(value: u32) -> bool {
  528|      3|        match Self::code_point_to_byte(value) {
  529|      2|            Some(value) => Self::is_printable_byte(value),
  530|      1|            None => false,
  531|       |        }
  532|      3|    }
  533|       |
  534|       |    /// Tests whether a byte is a Latin-1 control character.
  535|       |    ///
  536|       |    /// # Parameters
  537|       |    ///
  538|       |    /// - `value`: The byte to test.
  539|       |    ///
  540|       |    /// # Returns
  541|       |    ///
  542|       |    /// Returns `true` for C0 controls, DEL, or C1 controls.
  543|       |    #[inline(always)]
  544|       |    #[must_use]
  545|      8|    pub const fn is_control_byte(value: u8) -> bool {
  546|      8|        value < 0x20 || (value >= 0x7f && value <= 0x9f)
  547|      8|    }
  548|       |
  549|       |    /// Tests whether a character is a Latin-1 control character.
  550|       |    ///
  551|       |    /// # Parameters
  552|       |    ///
  553|       |    /// - `value`: The character to test.
  554|       |    ///
  555|       |    /// # Returns
  556|       |    ///
  557|       |    /// Returns `true` for C0 controls, DEL, or C1 controls.
  558|       |    #[inline]
  559|       |    #[must_use]
  560|      2|    pub const fn is_control_char(value: char) -> bool {
  561|      2|        match Self::char_to_byte(value) {
  562|      1|            Some(value) => Self::is_control_byte(value),
  563|      1|            None => false,
  564|       |        }
  565|      2|    }
  566|       |
  567|       |    /// Tests whether a raw code point is a Latin-1 control character.
  568|       |    ///
  569|       |    /// # Parameters
  570|       |    ///
  571|       |    /// - `value`: The raw code point value to test.
  572|       |    ///
  573|       |    /// # Returns
  574|       |    ///
  575|       |    /// Returns `true` for C0 controls, DEL, or C1 controls.
  576|       |    #[inline]
  577|       |    #[must_use]
  578|      2|    pub const fn is_control_code_point(value: u32) -> bool {
  579|      2|        match Self::code_point_to_byte(value) {
  580|      1|            Some(value) => Self::is_control_byte(value),
  581|      1|            None => false,
  582|       |        }
  583|      2|    }
  584|       |
  585|       |    /// Converts a Latin-1 byte to the corresponding Unicode character.
  586|       |    ///
  587|       |    /// # Parameters
  588|       |    ///
  589|       |    /// - `value`: The byte to convert.
  590|       |    ///
  591|       |    /// # Returns
  592|       |    ///
  593|       |    /// Returns the character with the same scalar value as `value`.
  594|       |    #[inline(always)]
  595|       |    #[must_use]
  596|     14|    pub const fn byte_to_char(value: u8) -> char {
  597|     14|        value as char
  598|     14|    }
  599|       |
  600|       |    /// Converts a Latin-1 byte to the corresponding code point.
  601|       |    ///
  602|       |    /// # Parameters
  603|       |    ///
  604|       |    /// - `value`: The byte to convert.
  605|       |    ///
  606|       |    /// # Returns
  607|       |    ///
  608|       |    /// Returns the code point with the same numeric value as `value`.
  609|       |    #[inline(always)]
  610|       |    #[must_use]
  611|      1|    pub const fn byte_to_code_point(value: u8) -> u32 {
  612|      1|        value as u32
  613|      1|    }
  614|       |
  615|       |    /// Compares two bytes while ignoring Latin-1 case.
  616|       |    ///
  617|       |    /// # Parameters
  618|       |    ///
  619|       |    /// - `left`: The first byte to compare.
  620|       |    /// - `right`: The second byte to compare.
  621|       |    ///
  622|       |    /// # Returns
  623|       |    ///
  624|       |    /// Returns `true` if both bytes match after Latin-1 lowercase conversion.
  625|       |    #[inline(always)]
  626|       |    #[must_use]
  627|      2|    pub const fn equals_ignore_case_byte(left: u8, right: u8) -> bool {
  628|      2|        if left == right {
  629|      1|            true
  630|       |        } else {
  631|      1|            Self::byte_to_lowercase(left) == Self::byte_to_lowercase(right)
  632|       |        }
  633|      2|    }
  634|       |
  635|       |    /// Compares two characters while ignoring Latin-1 case.
  636|       |    ///
  637|       |    /// # Parameters
  638|       |    ///
  639|       |    /// - `left`: The first character to compare.
  640|       |    /// - `right`: The second character to compare.
  641|       |    ///
  642|       |    /// # Returns
  643|       |    ///
  644|       |    /// Returns `true` if both Latin-1 characters match after lowercase
  645|       |    /// conversion. Non-Latin-1 characters are compared unchanged.
  646|       |    #[inline]
  647|       |    #[must_use]
  648|      3|    pub const fn equals_ignore_case_char(left: char, right: char) -> bool {
  649|      3|        if left == right {
  650|      1|            true
  651|       |        } else {
  652|      2|            Self::char_to_lowercase(left) == Self::char_to_lowercase(right)
  653|       |        }
  654|      3|    }
  655|       |
  656|       |    /// Compares two raw code points while ignoring Latin-1 case.
  657|       |    ///
  658|       |    /// # Parameters
  659|       |    ///
  660|       |    /// - `left`: The first raw code point value to compare.
  661|       |    /// - `right`: The second raw code point value to compare.
  662|       |    ///
  663|       |    /// # Returns
  664|       |    ///
  665|       |    /// Returns `true` if both values match after Latin-1 lowercase conversion.
  666|       |    #[inline]
  667|       |    #[must_use]
  668|      2|    pub const fn equals_ignore_case_code_point(left: u32, right: u32) -> bool {
  669|      2|        if left == right {
  670|      1|            true
  671|       |        } else {
  672|      1|            Self::code_point_to_lowercase(left) == Self::code_point_to_lowercase(right)
  673|       |        }
  674|      2|    }
  675|       |
  676|       |    /// Converts a byte to uppercase using Latin-1 case rules.
  677|       |    ///
  678|       |    /// # Parameters
  679|       |    ///
  680|       |    /// - `value`: The byte to convert.
  681|       |    ///
  682|       |    /// # Returns
  683|       |    ///
  684|       |    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
  685|       |    /// returns `value` unchanged.
  686|       |    #[inline(always)]
  687|       |    #[must_use]
  688|      6|    pub const fn byte_to_uppercase(value: u8) -> u8 {
  689|      6|        if Ascii::is_lowercase_letter_byte(value)
  690|      5|            || (value >= 0xe0 && value <= 0xf6)
                                               ^4
  691|      3|            || (value >= 0xf8 && value <= 0xfe)
                                               ^2
  692|       |        {
  693|      5|            value - 0x20
  694|       |        } else {
  695|      1|            value
  696|       |        }
  697|      6|    }
  698|       |
  699|       |    /// Converts a character to uppercase using Latin-1 case rules.
  700|       |    ///
  701|       |    /// # Parameters
  702|       |    ///
  703|       |    /// - `value`: The character to convert.
  704|       |    ///
  705|       |    /// # Returns
  706|       |    ///
  707|       |    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
  708|       |    /// returns `value` unchanged.
  709|       |    #[inline]
  710|       |    #[must_use]
  711|      2|    pub const fn char_to_uppercase(value: char) -> char {
  712|      2|        match Self::char_to_byte(value) {
  713|      1|            Some(value) => Self::byte_to_char(Self::byte_to_uppercase(value)),
  714|      1|            None => value,
  715|       |        }
  716|      2|    }
  717|       |
  718|       |    /// Converts a raw code point to uppercase using Latin-1 case rules.
  719|       |    ///
  720|       |    /// # Parameters
  721|       |    ///
  722|       |    /// - `value`: The raw code point value to convert.
  723|       |    ///
  724|       |    /// # Returns
  725|       |    ///
  726|       |    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
  727|       |    /// returns `value` unchanged.
  728|       |    #[inline]
  729|       |    #[must_use]
  730|      2|    pub const fn code_point_to_uppercase(value: u32) -> u32 {
  731|      2|        match Self::code_point_to_byte(value) {
  732|      1|            Some(value) => Self::byte_to_uppercase(value) as u32,
  733|      1|            None => value,
  734|       |        }
  735|      2|    }
  736|       |
  737|       |    /// Converts a byte to lowercase using Latin-1 case rules.
  738|       |    ///
  739|       |    /// # Parameters
  740|       |    ///
  741|       |    /// - `value`: The byte to convert.
  742|       |    ///
  743|       |    /// # Returns
  744|       |    ///
  745|       |    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
  746|       |    /// returns `value` unchanged.
  747|       |    #[inline(always)]
  748|       |    #[must_use]
  749|     12|    pub const fn byte_to_lowercase(value: u8) -> u8 {
  750|     12|        if Ascii::is_uppercase_letter_byte(value)
  751|     12|            || (value >= 0xc0 && value <= 0xd6)
  752|      8|            || (value >= 0xd8 && value <= 0xde)
                                               ^7
  753|       |        {
  754|      7|            value + 0x20
  755|       |        } else {
  756|      5|            value
  757|       |        }
  758|     12|    }
  759|       |
  760|       |    /// Converts a character to lowercase using Latin-1 case rules.
  761|       |    ///
  762|       |    /// # Parameters
  763|       |    ///
  764|       |    /// - `value`: The character to convert.
  765|       |    ///
  766|       |    /// # Returns
  767|       |    ///
  768|       |    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
  769|       |    /// returns `value` unchanged.
  770|       |    #[inline]
  771|       |    #[must_use]
  772|      6|    pub const fn char_to_lowercase(value: char) -> char {
  773|      6|        match Self::char_to_byte(value) {
  774|      5|            Some(value) => Self::byte_to_char(Self::byte_to_lowercase(value)),
  775|      1|            None => value,
  776|       |        }
  777|      6|    }
  778|       |
  779|       |    /// Converts a raw code point to lowercase using Latin-1 case rules.
  780|       |    ///
  781|       |    /// # Parameters
  782|       |    ///
  783|       |    /// - `value`: The raw code point value to convert.
  784|       |    ///
  785|       |    /// # Returns
  786|       |    ///
  787|       |    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
  788|       |    /// returns `value` unchanged.
  789|       |    #[inline]
  790|       |    #[must_use]
  791|      4|    pub const fn code_point_to_lowercase(value: u32) -> u32 {
  792|      4|        match Self::code_point_to_byte(value) {
  793|      3|            Some(value) => Self::byte_to_lowercase(value) as u32,
  794|      1|            None => value,
  795|       |        }
  796|      4|    }
  797|       |
  798|       |    /// Converts a character to a Latin-1 byte.
  799|       |    ///
  800|       |    /// # Parameters
  801|       |    ///
  802|       |    /// - `value`: The character to convert.
  803|       |    ///
  804|       |    /// # Returns
  805|       |    ///
  806|       |    /// Returns `Some(byte)` when `value` is in `U+0000..=U+00FF`; otherwise
  807|       |    /// returns `None`.
  808|       |    #[inline]
  809|       |    #[must_use]
  810|     40|    pub const fn char_to_byte(value: char) -> Option<u8> {
  811|     40|        if Self::is_latin1_char(value) {
  812|     25|            Some(value as u8)
  813|       |        } else {
  814|     15|            None
  815|       |        }
  816|     40|    }
  817|       |
  818|       |    /// Converts a raw code point to a Latin-1 byte.
  819|       |    ///
  820|       |    /// # Parameters
  821|       |    ///
  822|       |    /// - `value`: The raw code point value to convert.
  823|       |    ///
  824|       |    /// # Returns
  825|       |    ///
  826|       |    /// Returns `Some(byte)` when `value` is in `0x00..=0xFF`; otherwise
  827|       |    /// returns `None`.
  828|       |    #[inline]
  829|       |    #[must_use]
  830|     33|    pub const fn code_point_to_byte(value: u32) -> Option<u8> {
  831|     33|        if Self::is_latin1_code_point(value) {
  832|     18|            Some(value as u8)
  833|       |        } else {
  834|     15|            None
  835|       |        }
  836|     33|    }
  837|       |
  838|       |    /// Converts an ASCII decimal digit byte to its numeric value.
  839|       |    ///
  840|       |    /// # Parameters
  841|       |    ///
  842|       |    /// - `value`: The byte to convert.
  843|       |    ///
  844|       |    /// # Returns
  845|       |    ///
  846|       |    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
  847|       |    #[inline(always)]
  848|       |    #[must_use]
  849|      4|    pub const fn byte_to_digit(value: u8) -> Option<u8> {
  850|      4|        Ascii::byte_to_digit(value)
  851|      4|    }
  852|       |
  853|       |    /// Converts an ASCII decimal digit character to its numeric value.
  854|       |    ///
  855|       |    /// # Parameters
  856|       |    ///
  857|       |    /// - `value`: The character to convert.
  858|       |    ///
  859|       |    /// # Returns
  860|       |    ///
  861|       |    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
  862|       |    #[inline]
  863|       |    #[must_use]
  864|      2|    pub const fn char_to_digit(value: char) -> Option<u8> {
  865|      2|        match Self::char_to_byte(value) {
  866|      1|            Some(value) => Self::byte_to_digit(value),
  867|      1|            None => None,
  868|       |        }
  869|      2|    }
  870|       |
  871|       |    /// Converts an ASCII decimal digit code point to its numeric value.
  872|       |    ///
  873|       |    /// # Parameters
  874|       |    ///
  875|       |    /// - `value`: The raw code point value to convert.
  876|       |    ///
  877|       |    /// # Returns
  878|       |    ///
  879|       |    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
  880|       |    #[inline]
  881|       |    #[must_use]
  882|      2|    pub const fn code_point_to_digit(value: u32) -> Option<u8> {
  883|      2|        match Self::code_point_to_byte(value) {
  884|      1|            Some(value) => Self::byte_to_digit(value),
  885|      1|            None => None,
  886|       |        }
  887|      2|    }
  888|       |
  889|       |    /// Converts an ASCII hexadecimal digit byte to its numeric value.
  890|       |    ///
  891|       |    /// # Parameters
  892|       |    ///
  893|       |    /// - `value`: The byte to convert.
  894|       |    ///
  895|       |    /// # Returns
  896|       |    ///
  897|       |    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
  898|       |    /// `None`.
  899|       |    #[inline(always)]
  900|       |    #[must_use]
  901|      3|    pub const fn byte_to_hex_digit(value: u8) -> Option<u8> {
  902|      3|        Ascii::byte_to_hex_digit(value)
  903|      3|    }
  904|       |
  905|       |    /// Converts an ASCII hexadecimal digit character to its numeric value.
  906|       |    ///
  907|       |    /// # Parameters
  908|       |    ///
  909|       |    /// - `value`: The character to convert.
  910|       |    ///
  911|       |    /// # Returns
  912|       |    ///
  913|       |    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
  914|       |    /// `None`.
  915|       |    #[inline]
  916|       |    #[must_use]
  917|      3|    pub const fn char_to_hex_digit(value: char) -> Option<u8> {
  918|      3|        match Self::char_to_byte(value) {
  919|      2|            Some(value) => Self::byte_to_hex_digit(value),
  920|      1|            None => None,
  921|       |        }
  922|      3|    }
  923|       |
  924|       |    /// Converts an ASCII hexadecimal digit code point to its numeric value.
  925|       |    ///
  926|       |    /// # Parameters
  927|       |    ///
  928|       |    /// - `value`: The raw code point value to convert.
  929|       |    ///
  930|       |    /// # Returns
  931|       |    ///
  932|       |    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
  933|       |    /// `None`.
  934|       |    #[inline]
  935|       |    #[must_use]
  936|      2|    pub const fn code_point_to_hex_digit(value: u32) -> Option<u8> {
  937|      2|        match Self::code_point_to_byte(value) {
  938|      1|            Some(value) => Self::byte_to_hex_digit(value),
  939|      1|            None => None,
  940|       |        }
  941|      2|    }
  942|       |
  943|       |    /// Converts a raw code point to a Latin-1 character.
  944|       |    ///
  945|       |    /// # Parameters
  946|       |    ///
  947|       |    /// - `value`: The raw code point value to convert.
  948|       |    ///
  949|       |    /// # Returns
  950|       |    ///
  951|       |    /// Returns `Some(char)` when `value` is in `0x00..=0xFF`; otherwise
  952|       |    /// returns `None`.
  953|       |    #[inline]
  954|       |    #[must_use]
  955|      3|    pub const fn code_point_to_char(value: u32) -> Option<char> {
  956|      3|        if Self::is_latin1_code_point(value) {
  957|      2|            Some(Self::byte_to_char(value as u8))
  958|       |        } else {
  959|      1|            None
  960|       |        }
  961|      3|    }
  962|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/unicode.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |/// Namespace for Unicode constants and encoding-independent code point helpers.
    9|       |pub enum Unicode {}
   10|       |
   11|       |impl Unicode {
   12|       |    /// Maximum valid Unicode code point.
   13|       |    pub const MAX_CODE_POINT: u32 = 0x10ffff;
   14|       |
   15|       |    /// Unicode replacement character.
   16|       |    pub const REPLACEMENT_CHARACTER: char = '\u{fffd}';
   17|       |
   18|       |    /// Unicode byte order mark character.
   19|       |    pub const BOM: char = '\u{feff}';
   20|       |
   21|       |    /// Maximum valid ASCII code point.
   22|       |    pub const ASCII_MAX: u32 = 0x7f;
   23|       |
   24|       |    /// Maximum valid Latin-1 code point.
   25|       |    pub const LATIN1_MAX: u32 = 0xff;
   26|       |
   27|       |    /// Minimum supplementary code point.
   28|       |    pub const SUPPLEMENTARY_MIN: u32 = 0x10000;
   29|       |
   30|       |    /// Minimum high-surrogate code unit value.
   31|       |    pub const HIGH_SURROGATE_MIN: u32 = 0xd800;
   32|       |
   33|       |    /// Maximum high-surrogate code unit value.
   34|       |    pub const HIGH_SURROGATE_MAX: u32 = 0xdbff;
   35|       |
   36|       |    /// Minimum low-surrogate code unit value.
   37|       |    pub const LOW_SURROGATE_MIN: u32 = 0xdc00;
   38|       |
   39|       |    /// Maximum low-surrogate code unit value.
   40|       |    pub const LOW_SURROGATE_MAX: u32 = 0xdfff;
   41|       |
   42|       |    /// Minimum surrogate code unit value.
   43|       |    pub const SURROGATE_MIN: u32 = Self::HIGH_SURROGATE_MIN;
   44|       |
   45|       |    /// Maximum surrogate code unit value.
   46|       |    pub const SURROGATE_MAX: u32 = Self::LOW_SURROGATE_MAX;
   47|       |
   48|       |    /// Tests whether `value` is in the Unicode code point range.
   49|       |    ///
   50|       |    /// # Parameters
   51|       |    ///
   52|       |    /// - `value`: The raw code point value to test.
   53|       |    ///
   54|       |    /// # Returns
   55|       |    ///
   56|       |    /// Returns `true` for values in `0x0000..=0x10FFFF`, including surrogate
   57|       |    /// code points.
   58|       |    #[inline(always)]
   59|     20|    pub const fn is_code_point(value: u32) -> bool {
   60|     20|        value <= Self::MAX_CODE_POINT
   61|     20|    }
   62|       |
   63|       |    /// Tests whether `value` is a valid Unicode scalar value.
   64|       |    ///
   65|       |    /// # Parameters
   66|       |    ///
   67|       |    /// - `value`: The raw code point value to test.
   68|       |    ///
   69|       |    /// # Returns
   70|       |    ///
   71|       |    /// Returns `true` for Unicode code points excluding UTF-16 surrogate
   72|       |    /// values.
   73|       |    #[inline(always)]
   74|     11|    pub const fn is_scalar_value(value: u32) -> bool {
   75|     11|        Self::is_code_point(value) && !Self::is_surrogate(value)
                                                    ^8
   76|     11|    }
   77|       |
   78|       |    /// Tests whether `value` is a UTF-16 surrogate code point.
   79|       |    ///
   80|       |    /// # Parameters
   81|       |    ///
   82|       |    /// - `value`: The raw code point or code-unit value to test.
   83|       |    ///
   84|       |    /// # Returns
   85|       |    ///
   86|       |    /// Returns `true` if `value` is in `0xD800..=0xDFFF`.
   87|       |    #[inline(always)]
   88|      9|    pub const fn is_surrogate(value: u32) -> bool {
   89|      9|        value >= Self::SURROGATE_MIN && value <= Self::SURROGATE_MAX
                                                      ^7
   90|      9|    }
   91|       |
   92|       |    /// Tests whether `value` is a high surrogate.
   93|       |    ///
   94|       |    /// # Parameters
   95|       |    ///
   96|       |    /// - `value`: The raw code point or code-unit value to test.
   97|       |    ///
   98|       |    /// # Returns
   99|       |    ///
  100|       |    /// Returns `true` if `value` is in `0xD800..=0xDBFF`.
  101|       |    #[inline(always)]
  102|      1|    pub const fn is_high_surrogate(value: u32) -> bool {
  103|      1|        value >= Self::HIGH_SURROGATE_MIN && value <= Self::HIGH_SURROGATE_MAX
  104|      1|    }
  105|       |
  106|       |    /// Tests whether `value` is a low surrogate.
  107|       |    ///
  108|       |    /// # Parameters
  109|       |    ///
  110|       |    /// - `value`: The raw code point or code-unit value to test.
  111|       |    ///
  112|       |    /// # Returns
  113|       |    ///
  114|       |    /// Returns `true` if `value` is in `0xDC00..=0xDFFF`.
  115|       |    #[inline(always)]
  116|      1|    pub const fn is_low_surrogate(value: u32) -> bool {
  117|      1|        value >= Self::LOW_SURROGATE_MIN && value <= Self::LOW_SURROGATE_MAX
  118|      1|    }
  119|       |
  120|       |    /// Tests whether `value` is in the basic multilingual plane.
  121|       |    ///
  122|       |    /// # Parameters
  123|       |    ///
  124|       |    /// - `value`: The raw code point value to test.
  125|       |    ///
  126|       |    /// # Returns
  127|       |    ///
  128|       |    /// Returns `true` if `value` is in `0x0000..=0xFFFF`.
  129|       |    #[inline(always)]
  130|      2|    pub const fn is_bmp(value: u32) -> bool {
  131|      2|        value < Self::SUPPLEMENTARY_MIN
  132|      2|    }
  133|       |
  134|       |    /// Tests whether `value` is a supplementary Unicode code point.
  135|       |    ///
  136|       |    /// # Parameters
  137|       |    ///
  138|       |    /// - `value`: The raw code point value to test.
  139|       |    ///
  140|       |    /// # Returns
  141|       |    ///
  142|       |    /// Returns `true` if `value` is in `0x10000..=0x10FFFF`.
  143|       |    #[inline(always)]
  144|  8.08k|    pub const fn is_supplementary(value: u32) -> bool {
  145|  8.08k|        value >= Self::SUPPLEMENTARY_MIN && value <= Self::MAX_CODE_POINT
                                                          ^8.08k
  146|  8.08k|    }
  147|       |
  148|       |    /// Tests whether `value` is an ASCII code point.
  149|       |    ///
  150|       |    /// # Parameters
  151|       |    ///
  152|       |    /// - `value`: The raw code point value to test.
  153|       |    ///
  154|       |    /// # Returns
  155|       |    ///
  156|       |    /// Returns `true` if `value` is in `0x00..=0x7F`.
  157|       |    #[inline(always)]
  158|      2|    pub const fn is_ascii(value: u32) -> bool {
  159|      2|        value <= Self::ASCII_MAX
  160|      2|    }
  161|       |
  162|       |    /// Tests whether `value` is a Unicode noncharacter.
  163|       |    ///
  164|       |    /// # Parameters
  165|       |    ///
  166|       |    /// - `value`: The raw code point value to test.
  167|       |    ///
  168|       |    /// # Returns
  169|       |    ///
  170|       |    /// Returns `true` for Unicode noncharacters such as `U+FDD0..=U+FDEF` and
  171|       |    /// code points ending in `FFFE` or `FFFF`.
  172|       |    #[inline]
  173|      3|    pub const fn is_noncharacter(value: u32) -> bool {
  174|      3|        Self::is_code_point(value) && ((value >= 0xfdd0 && value <= 0xfdef) || (value & 0xfffe) == 0xfffe)
                                                                         ^2                  ^2
  175|      3|    }
  176|       |
  177|       |    /// Tests whether `value` is a Unicode C0, C1, or DEL control code point.
  178|       |    ///
  179|       |    /// # Parameters
  180|       |    ///
  181|       |    /// - `value`: The raw code point value to test.
  182|       |    ///
  183|       |    /// # Returns
  184|       |    ///
  185|       |    /// Returns `true` for `U+0000..=U+001F`, `U+007F`, and `U+0080..=U+009F`.
  186|       |    #[inline]
  187|      3|    pub const fn is_control(value: u32) -> bool {
  188|      3|        value <= 0x1f || (value >= 0x7f && value <= 0x9f)
                                        ^2               ^1
  189|      3|    }
  190|       |
  191|       |    /// Returns the Unicode plane containing `value`.
  192|       |    ///
  193|       |    /// # Parameters
  194|       |    ///
  195|       |    /// - `value`: The raw code point value to inspect.
  196|       |    ///
  197|       |    /// # Returns
  198|       |    ///
  199|       |    /// Returns `Some(plane)` for Unicode code points, or `None` for values
  200|       |    /// above `U+10FFFF`.
  201|       |    #[inline]
  202|      3|    pub const fn plane(value: u32) -> Option<u32> {
  203|      3|        if Self::is_code_point(value) {
  204|      2|            Some(value >> 16)
  205|       |        } else {
  206|      1|            None
  207|       |        }
  208|      3|    }
  209|       |
  210|       |    /// Converts a raw code point into a Rust `char`.
  211|       |    ///
  212|       |    /// # Parameters
  213|       |    ///
  214|       |    /// - `value`: The raw code point value to convert.
  215|       |    ///
  216|       |    /// # Returns
  217|       |    ///
  218|       |    /// Returns `Some(char)` for valid Unicode scalar values and `None`
  219|       |    /// otherwise.
  220|       |    #[inline]
  221|  53.1k|    pub fn to_char(value: u32) -> Option<char> {
  222|  53.1k|        char::from_u32(value)
  223|  53.1k|    }
  224|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/unicode_bom.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::ByteOrder;
    9|       |
   10|       |use crate::BomDetectStatus;
   11|       |use crate::Charset;
   12|       |
   13|       |/// Unicode byte order marks supported by this crate.
   14|       |///
   15|       |/// `detect` recognizes BOMs only from the bytes supplied to the call. Streaming
   16|       |/// callers should buffer up to four bytes, or read until EOF, before deciding
   17|       |/// that no longer BOM can be present.
   18|       |///
   19|       |/// # Examples
   20|       |///
   21|       |/// ```rust
   22|       |/// use qubit_codec::ByteOrder;
   23|       |/// use qubit_codec_text::{
   24|       |///     Charset,
   25|       |///     UnicodeBom,
   26|       |/// };
   27|       |///
   28|       |/// let bom = UnicodeBom::detect(&[0xff, 0xfe, 0x00, 0x00]);
   29|       |/// assert_eq!(Some(UnicodeBom::Utf32LittleEndian), bom);
   30|       |///
   31|       |/// let bom = bom.expect("UTF-32LE BOM");
   32|       |/// assert_eq!(Charset::UTF_32LE, bom.charset());
   33|       |/// assert_eq!(Some(ByteOrder::LittleEndian), bom.byte_order());
   34|       |/// assert_eq!(4, bom.byte_len());
   35|       |/// ```
   36|       |#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   37|       |pub enum UnicodeBom {
   38|       |    /// UTF-8 byte order mark.
   39|       |    Utf8,
   40|       |
   41|       |    /// UTF-16 big-endian byte order mark.
   42|       |    Utf16BigEndian,
   43|       |
   44|       |    /// UTF-16 little-endian byte order mark.
   45|       |    Utf16LittleEndian,
   46|       |
   47|       |    /// UTF-32 big-endian byte order mark.
   48|       |    Utf32BigEndian,
   49|       |
   50|       |    /// UTF-32 little-endian byte order mark.
   51|       |    Utf32LittleEndian,
   52|       |}
   53|       |
   54|       |impl UnicodeBom {
   55|       |    /// Supported BOMs in longest-prefix detection order.
   56|       |    const ALL: &'static [Self] = &[
   57|       |        Self::Utf32BigEndian,
   58|       |        Self::Utf32LittleEndian,
   59|       |        Self::Utf8,
   60|       |        Self::Utf16BigEndian,
   61|       |        Self::Utf16LittleEndian,
   62|       |    ];
   63|       |
   64|       |    /// Detects a Unicode byte order mark at the beginning of `bytes`.
   65|       |    ///
   66|       |    /// # Parameters
   67|       |    ///
   68|       |    /// - `bytes`: The byte buffer to inspect.
   69|       |    ///
   70|       |    /// # Returns
   71|       |    ///
   72|       |    /// Returns the detected BOM, or `None` if no supported BOM prefix is
   73|       |    /// present.
   74|       |    ///
   75|       |    /// UTF-32 BOMs are checked before UTF-16 BOMs so that overlapping prefixes
   76|       |    /// such as `FF FE 00 00` are classified as UTF-32 little-endian when all
   77|       |    /// four bytes are available.
   78|     17|    pub fn detect(bytes: &[u8]) -> Option<Self> {
   79|     17|        match Self::detect_progress(bytes, true) {
   80|     15|            BomDetectStatus::Match(bom) => Some(bom),
   81|      2|            BomDetectStatus::Pending | BomDetectStatus::None => None,
   82|       |        }
   83|     17|    }
   84|       |
   85|       |    /// Detects a Unicode byte order mark with an explicit EOF signal.
   86|       |    ///
   87|       |    /// # Parameters
   88|       |    ///
   89|       |    /// - `bytes`: Bytes currently available from the beginning of the stream.
   90|       |    /// - `eof`: Whether no more bytes can arrive.
   91|       |    ///
   92|       |    /// # Returns
   93|       |    ///
   94|       |    /// Returns [`BomDetectStatus::Pending`] when the bytes are a valid prefix
   95|       |    /// of a longer BOM and EOF has not been reached, [`BomDetectStatus::Match`]
   96|       |    /// when a supported BOM can be selected, or [`BomDetectStatus::None`] when
   97|       |    /// no supported BOM can match.
   98|     38|    pub fn detect_progress(bytes: &[u8], eof: bool) -> BomDetectStatus {
   99|     38|        if let Some(bom) = detect_complete(bytes) {
                                  ^26
  100|     26|            if !eof && has_longer_possible_match(bytes, bom) {
                                     ^7                        ^7     ^7
  101|      3|                BomDetectStatus::Pending
  102|       |            } else {
  103|     23|                BomDetectStatus::Match(bom)
  104|       |            }
  105|     12|        } else if !eof && is_possible_bom_prefix(bytes) {
                                        ^6                     ^6
  106|      4|            BomDetectStatus::Pending
  107|       |        } else {
  108|      8|            BomDetectStatus::None
  109|       |        }
  110|     38|    }
  111|       |
  112|       |    /// Returns the bytes that represent this BOM.
  113|       |    ///
  114|       |    /// # Returns
  115|       |    ///
  116|       |    /// Returns a static byte slice containing the BOM bytes.
  117|       |    #[inline]
  118|    193|    pub const fn bytes(self) -> &'static [u8] {
  119|    193|        match self {
  120|     39|            Self::Utf8 => &[0xef, 0xbb, 0xbf],
  121|     30|            Self::Utf16BigEndian => &[0xfe, 0xff],
  122|     26|            Self::Utf16LittleEndian => &[0xff, 0xfe],
  123|     51|            Self::Utf32BigEndian => &[0x00, 0x00, 0xfe, 0xff],
  124|     47|            Self::Utf32LittleEndian => &[0xff, 0xfe, 0x00, 0x00],
  125|       |        }
  126|    193|    }
  127|       |
  128|       |    /// Returns the byte length of this BOM.
  129|       |    ///
  130|       |    /// # Returns
  131|       |    ///
  132|       |    /// Returns the number of bytes in this BOM.
  133|       |    #[inline]
  134|     57|    pub const fn byte_len(self) -> usize {
  135|     57|        match self {
  136|     15|            Self::Utf8 => 3,
  137|     21|            Self::Utf16BigEndian | Self::Utf16LittleEndian => 2,
  138|     21|            Self::Utf32BigEndian | Self::Utf32LittleEndian => 4,
  139|       |        }
  140|     57|    }
  141|       |
  142|       |    /// Returns the charset indicated by this BOM.
  143|       |    ///
  144|       |    /// # Returns
  145|       |    ///
  146|       |    /// Returns the corresponding [`Charset`], including fixed byte order for
  147|       |    /// UTF-16 and UTF-32 BOMs.
  148|       |    #[inline]
  149|      5|    pub const fn charset(self) -> Charset {
  150|      5|        match self {
  151|      1|            Self::Utf8 => Charset::UTF_8,
  152|      1|            Self::Utf16BigEndian => Charset::UTF_16BE,
  153|      1|            Self::Utf16LittleEndian => Charset::UTF_16LE,
  154|      1|            Self::Utf32BigEndian => Charset::UTF_32BE,
  155|      1|            Self::Utf32LittleEndian => Charset::UTF_32LE,
  156|       |        }
  157|      5|    }
  158|       |
  159|       |    /// Returns the byte order indicated by this BOM when applicable.
  160|       |    ///
  161|       |    /// # Returns
  162|       |    ///
  163|       |    /// Returns `Some(ByteOrder)` for UTF-16 and UTF-32 BOMs. Returns `None` for
  164|       |    /// UTF-8 because byte order does not apply.
  165|       |    #[inline]
  166|      7|    pub const fn byte_order(self) -> Option<ByteOrder> {
  167|      7|        match self {
  168|      2|            Self::Utf8 => None,
  169|      2|            Self::Utf16BigEndian | Self::Utf32BigEndian => Some(ByteOrder::BigEndian),
  170|      3|            Self::Utf16LittleEndian | Self::Utf32LittleEndian => Some(ByteOrder::LittleEndian),
  171|       |        }
  172|      7|    }
  173|       |}
  174|       |
  175|       |/// Detects a complete BOM using longest-prefix order.
  176|       |///
  177|       |/// # Parameters
  178|       |///
  179|       |/// - `bytes`: Candidate stream prefix.
  180|       |///
  181|       |/// # Returns
  182|       |///
  183|       |/// Returns the longest supported BOM that is fully present in `bytes`.
  184|       |#[inline]
  185|     38|fn detect_complete(bytes: &[u8]) -> Option<UnicodeBom> {
  186|     38|    UnicodeBom::ALL
  187|     38|        .iter()
  188|     38|        .copied()
  189|    152|        .find(|bom| bytes.starts_with(bom.bytes()))
                       ^38
  190|     38|}
  191|       |
  192|       |/// Tests whether `bytes` may still become a supported BOM.
  193|       |///
  194|       |/// # Parameters
  195|       |///
  196|       |/// - `bytes`: Bytes currently available from the beginning of the stream.
  197|       |///
  198|       |/// # Returns
  199|       |///
  200|       |/// Returns `true` when `bytes` is a prefix of any supported BOM.
  201|       |#[inline]
  202|      6|fn is_possible_bom_prefix(bytes: &[u8]) -> bool {
  203|     19|    UnicodeBom::ALL.iter().any(|bom| bom.bytes().starts_with(bytes))
                  ^6                     ^6
  204|      6|}
  205|       |
  206|       |/// Tests whether a currently matched BOM is ambiguous with a longer BOM.
  207|       |///
  208|       |/// # Parameters
  209|       |///
  210|       |/// - `bytes`: Bytes currently available from the beginning of the stream.
  211|       |/// - `matched`: BOM selected from the currently available bytes.
  212|       |///
  213|       |/// # Returns
  214|       |///
  215|       |/// Returns `true` when additional bytes could promote the match to a longer
  216|       |/// BOM.
  217|       |#[inline]
  218|      7|fn has_longer_possible_match(bytes: &[u8], matched: UnicodeBom) -> bool {
  219|      7|    UnicodeBom::ALL
  220|      7|        .iter()
  221|     26|        .any(|bom| bom.byte_len() > matched.byte_len() && bom.bytes().starts_with(bytes))
                       ^7                                               ^13         ^13         ^13
  222|      7|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/utf16.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::ByteOrder;
    9|       |
   10|       |use crate::Unicode;
   11|       |use crate::UnicodeBom;
   12|       |
   13|       |/// Namespace for UTF-16 constants and code-unit classification helpers.
   14|       |pub enum Utf16 {}
   15|       |
   16|       |impl Utf16 {
   17|       |    /// Maximum number of UTF-16 code units needed for one Unicode scalar value.
   18|       |    pub const MAX_UNITS_PER_CHAR: usize = 2;
   19|       |
   20|       |    /// Maximum number of serialized UTF-16 bytes needed for one Unicode scalar
   21|       |    /// value.
   22|       |    pub const MAX_BYTES_PER_CHAR: usize = 4;
   23|       |
   24|       |    /// Tests whether a UTF-16 unit is a high surrogate.
   25|       |    ///
   26|       |    /// # Parameters
   27|       |    ///
   28|       |    /// - `unit`: The UTF-16 unit to test.
   29|       |    ///
   30|       |    /// # Returns
   31|       |    ///
   32|       |    /// Returns `true` if `unit` is in `0xD800..=0xDBFF`.
   33|       |    #[inline(always)]
   34|  20.2k|    pub const fn is_high_surrogate(unit: u16) -> bool {
   35|  20.2k|        (unit as u32) >= Unicode::HIGH_SURROGATE_MIN && (unit as u32) <= Unicode::HIGH_SURROGATE_MAX
                                                                      ^8.68k
   36|  20.2k|    }
   37|       |
   38|       |    /// Tests whether a UTF-16 unit is a low surrogate.
   39|       |    ///
   40|       |    /// # Parameters
   41|       |    ///
   42|       |    /// - `unit`: The UTF-16 unit to test.
   43|       |    ///
   44|       |    /// # Returns
   45|       |    ///
   46|       |    /// Returns `true` if `unit` is in `0xDC00..=0xDFFF`.
   47|       |    #[inline(always)]
   48|  16.2k|    pub const fn is_low_surrogate(unit: u16) -> bool {
   49|  16.2k|        (unit as u32) >= Unicode::LOW_SURROGATE_MIN && (unit as u32) <= Unicode::LOW_SURROGATE_MAX
                                                                     ^4.64k
   50|  16.2k|    }
   51|       |
   52|       |    /// Tests whether a UTF-16 unit is any surrogate.
   53|       |    ///
   54|       |    /// # Parameters
   55|       |    ///
   56|       |    /// - `unit`: The UTF-16 unit to test.
   57|       |    ///
   58|       |    /// # Returns
   59|       |    ///
   60|       |    /// Returns `true` if `unit` is in `0xD800..=0xDFFF`.
   61|       |    #[inline(always)]
   62|      3|    pub const fn is_surrogate(unit: u16) -> bool {
   63|      3|        (unit as u32) >= Unicode::SURROGATE_MIN && (unit as u32) <= Unicode::SURROGATE_MAX
                                                                 ^2
   64|      3|    }
   65|       |
   66|       |    /// Tests whether a UTF-16 unit encodes a scalar value by itself.
   67|       |    ///
   68|       |    /// # Parameters
   69|       |    ///
   70|       |    /// - `unit`: The UTF-16 unit to test.
   71|       |    ///
   72|       |    /// # Returns
   73|       |    ///
   74|       |    /// Returns `true` for non-surrogate units.
   75|       |    #[inline(always)]
   76|      2|    pub const fn is_single_unit(unit: u16) -> bool {
   77|      2|        !Self::is_surrogate(unit)
   78|      2|    }
   79|       |
   80|       |    /// Tests whether two UTF-16 units form a surrogate pair.
   81|       |    ///
   82|       |    /// # Parameters
   83|       |    ///
   84|       |    /// - `high`: The candidate high surrogate.
   85|       |    /// - `low`: The candidate low surrogate.
   86|       |    ///
   87|       |    /// # Returns
   88|       |    ///
   89|       |    /// Returns `true` if `high` is a high surrogate and `low` is a low
   90|       |    /// surrogate.
   91|       |    #[inline(always)]
   92|  4.02k|    pub const fn is_surrogate_pair(high: u16, low: u16) -> bool {
   93|  4.02k|        Self::is_high_surrogate(high) && Self::is_low_surrogate(low)
                                                       ^4.02k                 ^4.02k
   94|  4.02k|    }
   95|       |
   96|       |    /// Returns the UTF-16 unit count needed for `ch`.
   97|       |    ///
   98|       |    /// # Parameters
   99|       |    ///
  100|       |    /// - `ch`: The character to size.
  101|       |    ///
  102|       |    /// # Returns
  103|       |    ///
  104|       |    /// Returns `1` for BMP scalar values and `2` for supplementary scalar
  105|       |    /// values.
  106|       |    #[inline(always)]
  107|  32.9k|    pub const fn unit_len(ch: char) -> usize {
  108|  32.9k|        if (ch as u32) >= Unicode::SUPPLEMENTARY_MIN {
  109|  8.07k|            2
  110|       |        } else {
  111|  24.8k|            1
  112|       |        }
  113|  32.9k|    }
  114|       |
  115|       |    /// Returns the UTF-16 unit count needed for a raw code point.
  116|       |    ///
  117|       |    /// # Parameters
  118|       |    ///
  119|       |    /// - `code_point`: The raw code point to size.
  120|       |    ///
  121|       |    /// # Returns
  122|       |    ///
  123|       |    /// Returns `Some(1)` or `Some(2)` for scalar values and `None` otherwise.
  124|       |    #[inline]
  125|      4|    pub const fn unit_len_code_point(code_point: u32) -> Option<usize> {
  126|      4|        if !Unicode::is_scalar_value(code_point) {
  127|      2|            None
  128|      2|        } else if code_point >= Unicode::SUPPLEMENTARY_MIN {
  129|      1|            Some(2)
  130|       |        } else {
  131|      1|            Some(1)
  132|       |        }
  133|      4|    }
  134|       |
  135|       |    /// Composes a surrogate pair into a Unicode code point.
  136|       |    ///
  137|       |    /// # Parameters
  138|       |    ///
  139|       |    /// - `high`: The high surrogate.
  140|       |    /// - `low`: The low surrogate.
  141|       |    ///
  142|       |    /// # Returns
  143|       |    ///
  144|       |    /// Returns `Some(code_point)` when the pair is valid, or `None` otherwise.
  145|       |    #[inline]
  146|  4.02k|    pub const fn compose_pair(high: u16, low: u16) -> Option<u32> {
  147|  4.02k|        if Self::is_surrogate_pair(high, low) {
  148|  4.01k|            let high_payload = (high as u32) - Unicode::HIGH_SURROGATE_MIN;
  149|  4.01k|            let low_payload = (low as u32) - Unicode::LOW_SURROGATE_MIN;
  150|  4.01k|            Some((high_payload << 10) + low_payload + Unicode::SUPPLEMENTARY_MIN)
  151|       |        } else {
  152|      7|            None
  153|       |        }
  154|  4.02k|    }
  155|       |
  156|       |    /// Returns the high surrogate for a supplementary code point.
  157|       |    ///
  158|       |    /// # Parameters
  159|       |    ///
  160|       |    /// - `code_point`: The supplementary code point.
  161|       |    ///
  162|       |    /// # Returns
  163|       |    ///
  164|       |    /// Returns `Some(high_surrogate)` for supplementary code points and `None`
  165|       |    /// otherwise.
  166|       |    #[inline]
  167|  4.04k|    pub const fn high_surrogate(code_point: u32) -> Option<u16> {
  168|  4.04k|        if Unicode::is_supplementary(code_point) {
  169|  4.03k|            Some((((code_point - Unicode::SUPPLEMENTARY_MIN) >> 10) + Unicode::HIGH_SURROGATE_MIN) as u16)
  170|       |        } else {
  171|      1|            None
  172|       |        }
  173|  4.04k|    }
  174|       |
  175|       |    /// Returns the low surrogate for a supplementary code point.
  176|       |    ///
  177|       |    /// # Parameters
  178|       |    ///
  179|       |    /// - `code_point`: The supplementary code point.
  180|       |    ///
  181|       |    /// # Returns
  182|       |    ///
  183|       |    /// Returns `Some(low_surrogate)` for supplementary code points and `None`
  184|       |    /// otherwise.
  185|       |    #[inline]
  186|  4.04k|    pub const fn low_surrogate(code_point: u32) -> Option<u16> {
  187|  4.04k|        if Unicode::is_supplementary(code_point) {
  188|  4.03k|            Some((((code_point - Unicode::SUPPLEMENTARY_MIN) & 0x3ff) + Unicode::LOW_SURROGATE_MIN) as u16)
  189|       |        } else {
  190|      1|            None
  191|       |        }
  192|  4.04k|    }
  193|       |
  194|       |    /// Detects a UTF-16 BOM and returns its byte order.
  195|       |    ///
  196|       |    /// # Parameters
  197|       |    ///
  198|       |    /// - `bytes`: The byte buffer to inspect.
  199|       |    ///
  200|       |    /// # Returns
  201|       |    ///
  202|       |    /// Returns `Some(ByteOrder)` for UTF-16 BOM prefixes, or `None` otherwise.
  203|      3|    pub fn detect_bom(bytes: &[u8]) -> Option<ByteOrder> {
  204|      3|        match UnicodeBom::detect(bytes) {
  205|      1|            Some(UnicodeBom::Utf16BigEndian) => Some(ByteOrder::BigEndian),
  206|      1|            Some(UnicodeBom::Utf16LittleEndian) => Some(ByteOrder::LittleEndian),
  207|      1|            _ => None,
  208|       |        }
  209|      3|    }
  210|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/utf32.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::ByteOrder;
    9|       |
   10|       |use crate::Unicode;
   11|       |use crate::UnicodeBom;
   12|       |
   13|       |/// Namespace for UTF-32 constants and code-unit classification helpers.
   14|       |pub enum Utf32 {}
   15|       |
   16|       |impl Utf32 {
   17|       |    /// Maximum number of UTF-32 code units needed for one Unicode scalar value.
   18|       |    pub const MAX_UNITS_PER_CHAR: usize = 1;
   19|       |
   20|       |    /// Maximum number of serialized UTF-32 bytes needed for one Unicode scalar
   21|       |    /// value.
   22|       |    pub const MAX_BYTES_PER_CHAR: usize = 4;
   23|       |
   24|       |    /// Tests whether a UTF-32 unit is a valid Unicode scalar value.
   25|       |    ///
   26|       |    /// # Parameters
   27|       |    ///
   28|       |    /// - `unit`: The UTF-32 unit to test.
   29|       |    ///
   30|       |    /// # Returns
   31|       |    ///
   32|       |    /// Returns `true` if `unit` is a valid Unicode scalar value.
   33|       |    #[inline(always)]
   34|      3|    pub const fn is_valid_unit(unit: u32) -> bool {
   35|      3|        Unicode::is_scalar_value(unit)
   36|      3|    }
   37|       |
   38|       |    /// Returns the UTF-32 unit count needed for a character.
   39|       |    ///
   40|       |    /// # Parameters
   41|       |    ///
   42|       |    /// - `_ch`: The character to size.
   43|       |    ///
   44|       |    /// # Returns
   45|       |    ///
   46|       |    /// Always returns `1`.
   47|       |    #[inline(always)]
   48|      1|    pub const fn unit_len(_ch: char) -> usize {
   49|      1|        1
   50|      1|    }
   51|       |
   52|       |    /// Detects a UTF-32 BOM and returns its byte order.
   53|       |    ///
   54|       |    /// # Parameters
   55|       |    ///
   56|       |    /// - `bytes`: The byte buffer to inspect.
   57|       |    ///
   58|       |    /// # Returns
   59|       |    ///
   60|       |    /// Returns `Some(ByteOrder)` for UTF-32 BOM prefixes, or `None` otherwise.
   61|      3|    pub fn detect_bom(bytes: &[u8]) -> Option<ByteOrder> {
   62|      3|        match UnicodeBom::detect(bytes) {
   63|      1|            Some(UnicodeBom::Utf32BigEndian) => Some(ByteOrder::BigEndian),
   64|      1|            Some(UnicodeBom::Utf32LittleEndian) => Some(ByteOrder::LittleEndian),
   65|      1|            _ => None,
   66|       |        }
   67|      3|    }
   68|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/charset/utf8.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |/// Namespace for UTF-8 constants and byte classification helpers.
    9|       |pub enum Utf8 {}
   10|       |
   11|       |impl Utf8 {
   12|       |    /// Maximum number of UTF-8 bytes needed for one Unicode scalar value.
   13|       |    pub const MAX_UNITS_PER_CHAR: usize = 4;
   14|       |
   15|       |    /// Maximum number of UTF-8 bytes needed for one Unicode scalar value.
   16|       |    pub const MAX_BYTES_PER_CHAR: usize = Self::MAX_UNITS_PER_CHAR;
   17|       |
   18|       |    /// Tests whether a byte encodes an ASCII scalar value by itself.
   19|       |    ///
   20|       |    /// # Parameters
   21|       |    ///
   22|       |    /// - `byte`: The byte to test.
   23|       |    ///
   24|       |    /// # Returns
   25|       |    ///
   26|       |    /// Returns `true` if `byte` is in `0x00..=0x7F`.
   27|       |    #[inline(always)]
   28|      2|    pub const fn is_single_byte(byte: u8) -> bool {
   29|      2|        byte <= 0x7f
   30|      2|    }
   31|       |
   32|       |    /// Tests whether a byte can lead a multi-byte UTF-8 sequence.
   33|       |    ///
   34|       |    /// # Parameters
   35|       |    ///
   36|       |    /// - `byte`: The byte to test.
   37|       |    ///
   38|       |    /// # Returns
   39|       |    ///
   40|       |    /// Returns `true` if `byte` is in `0xC2..=0xF4`.
   41|       |    #[inline(always)]
   42|      3|    pub const fn is_leading_byte(byte: u8) -> bool {
   43|      3|        byte >= 0xc2 && byte <= 0xf4
                                      ^2
   44|      3|    }
   45|       |
   46|       |    /// Tests whether a byte is a UTF-8 continuation byte.
   47|       |    ///
   48|       |    /// # Parameters
   49|       |    ///
   50|       |    /// - `byte`: The byte to test.
   51|       |    ///
   52|       |    /// # Returns
   53|       |    ///
   54|       |    /// Returns `true` if `byte` matches the `10xxxxxx` continuation pattern.
   55|       |    #[inline(always)]
   56|  26.3k|    pub const fn is_continuation_byte(byte: u8) -> bool {
   57|  26.3k|        (byte & 0xc0) == 0x80
   58|  26.3k|    }
   59|       |
   60|       |    /// Returns the complete UTF-8 sequence length implied by a leading byte.
   61|       |    ///
   62|       |    /// # Parameters
   63|       |    ///
   64|       |    /// - `byte`: The first byte of the UTF-8 sequence.
   65|       |    ///
   66|       |    /// # Returns
   67|       |    ///
   68|       |    /// Returns `Some(1..=4)` for valid ASCII or leading bytes, and `None` for
   69|       |    /// continuation bytes or invalid leading bytes.
   70|       |    #[inline(always)]
   71|  74.9k|    pub const fn byte_len_from_leading_byte(byte: u8) -> Option<usize> {
   72|  74.9k|        if byte <= 0x7f {
   73|  40.4k|            Some(1)
   74|  34.4k|        } else if byte >= 0xc2 && byte <= 0xdf {
                                                ^21.3k
   75|  8.57k|            Some(2)
   76|  25.9k|        } else if byte >= 0xe0 && byte <= 0xef {
                                                ^12.7k
   77|  4.90k|            Some(3)
   78|  21.0k|        } else if byte >= 0xf0 && byte <= 0xf4 {
                                                ^7.86k
   79|  5.09k|            Some(4)
   80|       |        } else {
   81|  15.9k|            None
   82|       |        }
   83|  74.9k|    }
   84|       |
   85|       |    /// Returns the number of UTF-8 bytes needed for `ch`.
   86|       |    ///
   87|       |    /// # Parameters
   88|       |    ///
   89|       |    /// - `ch`: The character to size.
   90|       |    ///
   91|       |    /// # Returns
   92|       |    ///
   93|       |    /// Returns `1`, `2`, `3`, or `4`.
   94|       |    #[inline(always)]
   95|  46.2k|    pub const fn byte_len(ch: char) -> usize {
   96|  46.2k|        let code_point = ch as u32;
   97|  46.2k|        if code_point <= 0x7f {
   98|  27.5k|            1
   99|  18.7k|        } else if code_point <= 0x7ff {
  100|  3.79k|            2
  101|  14.9k|        } else if code_point <= 0xffff {
  102|  3.30k|            3
  103|       |        } else {
  104|  11.6k|            4
  105|       |        }
  106|  46.2k|    }
  107|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/ascii_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::num::NonZeroUsize;
    9|       |
   10|       |use qubit_codec::Codec;
   11|       |use qubit_utils::UncheckedSlice;
   12|       |
   13|       |use crate::Ascii;
   14|       |use crate::Charset;
   15|       |use crate::CharsetCodec;
   16|       |use crate::CharsetDecodeError;
   17|       |use crate::CharsetDecodeErrorKind;
   18|       |use crate::CharsetEncodeError;
   19|       |use crate::CharsetEncodeResult;
   20|       |use crate::error::CharsetCodecDecodeResult;
   21|       |
   22|       |/// Single-byte ASCII codec for bytes.
   23|       |///
   24|       |/// `AsciiCodec` converts between one-byte ASCII-encoded data and Unicode scalar
   25|       |/// values.
   26|       |#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
   27|       |pub struct AsciiCodec;
   28|       |
   29|       |impl AsciiCodec {
   30|       |    /// Returns the ASCII charset descriptor.
   31|       |    ///
   32|       |    /// # Returns
   33|       |    ///
   34|       |    /// Returns [`Charset::ASCII`].
   35|       |    #[inline]
   36|       |    #[must_use]
   37|      3|    pub const fn charset(self) -> Charset {
   38|      3|        Charset::ASCII
   39|      3|    }
   40|       |}
   41|       |
   42|       |impl CharsetCodec for AsciiCodec {
   43|       |    /// Returns the charset descriptor for this codec.
   44|       |    ///
   45|       |    /// # Returns
   46|       |    ///
   47|       |    /// Returns [`Charset::ASCII`].
   48|       |    #[inline]
   49|      8|    fn charset(&self) -> Charset {
   50|      8|        Charset::ASCII
   51|      8|    }
   52|       |}
   53|       |
   54|       |impl Codec for AsciiCodec {
   55|       |    type Value = char;
   56|       |    type Unit = u8;
   57|       |    type DecodeError = CharsetDecodeError;
   58|       |    type EncodeError = CharsetEncodeError;
   59|       |
   60|       |    const MIN_UNITS_PER_VALUE: usize = 1;
   61|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;
   62|       |
   63|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;
   64|       |
   65|       |    #[inline]
   66|     41|    fn can_encode_value(&self, value: &char) -> bool {
   67|     41|        Ascii::is_ascii_char(*value)
   68|     41|    }
   69|       |
   70|       |    #[inline]
   71|      5|    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
   72|      5|        debug_assert!(input_index < input.len());
   73|       |
   74|       |        // SAFETY: The caller guarantees that `input_index` is readable.
   75|      5|        let value = unsafe { UncheckedSlice::read(input, input_index) };
   76|      5|        if !Ascii::is_ascii_byte(value) {
   77|      2|            let kind = CharsetDecodeErrorKind::malformed(value as u32);
   78|      2|            return Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure());
   79|      3|        }
   80|      3|        Ok((value as char, NonZeroUsize::MIN))
   81|      5|    }
   82|       |
   83|       |    #[inline]
   84|     15|    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
   85|     15|        debug_assert!(self.can_encode_value(ch));
   86|     15|        debug_assert!(output_index < output.len());
   87|       |
   88|       |        // SAFETY: The caller guarantees that `ch` is encodable and
   89|       |        // `output_index` is writable.
   90|     15|        unsafe {
   91|     15|            UncheckedSlice::write(output, output_index, *ch as u8);
   92|     15|        }
   93|     15|        Ok(1)
   94|     15|    }
   95|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/latin1_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::num::NonZeroUsize;
    9|       |
   10|       |use qubit_codec::Codec;
   11|       |use qubit_utils::UncheckedSlice;
   12|       |
   13|       |use crate::Charset;
   14|       |use crate::CharsetCodec;
   15|       |use crate::CharsetDecodeError;
   16|       |use crate::CharsetEncodeError;
   17|       |use crate::CharsetEncodeResult;
   18|       |use crate::Latin1;
   19|       |use crate::error::CharsetCodecDecodeResult;
   20|       |
   21|       |/// Single-byte ISO-8859-1 codec for bytes.
   22|       |///
   23|       |/// `Latin1Codec` converts between ISO-8859-1 bytes and Unicode scalar values.
   24|       |#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
   25|       |pub struct Latin1Codec;
   26|       |
   27|       |impl Latin1Codec {
   28|       |    /// Returns the ISO-8859-1 charset descriptor.
   29|       |    ///
   30|       |    /// # Returns
   31|       |    ///
   32|       |    /// Returns [`Charset::ISO_8859_1`].
   33|       |    #[inline]
   34|       |    #[must_use]
   35|      3|    pub const fn charset(self) -> Charset {
   36|      3|        Charset::ISO_8859_1
   37|      3|    }
   38|       |}
   39|       |
   40|       |impl CharsetCodec for Latin1Codec {
   41|       |    /// Returns the charset descriptor for this codec.
   42|       |    ///
   43|       |    /// # Returns
   44|       |    ///
   45|       |    /// Returns [`Charset::ISO_8859_1`].
   46|       |    #[inline]
   47|      2|    fn charset(&self) -> Charset {
   48|      2|        Charset::ISO_8859_1
   49|      2|    }
   50|       |}
   51|       |
   52|       |impl Codec for Latin1Codec {
   53|       |    type Value = char;
   54|       |    type Unit = u8;
   55|       |    type DecodeError = CharsetDecodeError;
   56|       |    type EncodeError = CharsetEncodeError;
   57|       |
   58|       |    const MIN_UNITS_PER_VALUE: usize = 1;
   59|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;
   60|       |
   61|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;
   62|       |
   63|       |    #[inline]
   64|      9|    fn can_encode_value(&self, value: &char) -> bool {
   65|      9|        Latin1::is_latin1_char(*value)
   66|      9|    }
   67|       |
   68|       |    #[inline]
   69|      5|    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
   70|      5|        debug_assert!(input_index < input.len());
   71|       |        // SAFETY: The caller guarantees that `input_index` is readable.
   72|      5|        let value = unsafe { UncheckedSlice::read(input, input_index) };
   73|      5|        Ok((Latin1::byte_to_char(value), NonZeroUsize::MIN))
   74|      5|    }
   75|       |
   76|       |    #[inline]
   77|      3|    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
   78|      3|        debug_assert!(self.can_encode_value(ch));
   79|      3|        debug_assert!(output_index < output.len());
   80|       |
   81|      3|        let value = Latin1::char_to_byte(*ch).expect("encodable Latin-1 character maps to byte");
   82|       |        // SAFETY: The caller guarantees that `ch` is encodable and
   83|       |        // `output_index` is writable.
   84|      3|        unsafe {
   85|      3|            UncheckedSlice::write(output, output_index, value);
   86|      3|        }
   87|      3|        Ok(1)
   88|      3|    }
   89|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/utf16_byte_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::num::NonZeroUsize;
    9|       |
   10|       |use qubit_codec::ByteOrder;
   11|       |use qubit_codec::Codec;
   12|       |use qubit_utils::SliceRange;
   13|       |use qubit_utils::UncheckedSlice;
   14|       |use qubit_utils::nonzero;
   15|       |
   16|       |use crate::Charset;
   17|       |use crate::CharsetCodec;
   18|       |use crate::CharsetDecodeError;
   19|       |use crate::CharsetDecodeErrorKind;
   20|       |use crate::CharsetDecodeResult;
   21|       |use crate::CharsetEncodeError;
   22|       |use crate::CharsetEncodeResult;
   23|       |use crate::Unicode;
   24|       |use crate::Utf16;
   25|       |use crate::error::CharsetCodecDecodeResult;
   26|       |
   27|       |/// Combined byte-serialized UTF-16 codec.
   28|       |///
   29|       |/// The codec uses one configured byte order for both decoding and encoding. It
   30|       |/// does not detect, consume, or emit a BOM automatically; callers should use
   31|       |/// [`crate::UnicodeBom`] when a byte stream may carry an explicit BOM.
   32|       |///
   33|       |/// # Examples
   34|       |///
   35|       |/// ```rust
   36|       |/// use qubit_codec::{
   37|       |///     ByteOrder,
   38|       |///     Codec,
   39|       |/// };
   40|       |/// use qubit_codec_text::{
   41|       |///     CharsetCodec,
   42|       |///     Charset,
   43|       |///     Utf16,
   44|       |///     Utf16ByteCodec,
   45|       |/// };
   46|       |///
   47|       |/// let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
   48|       |/// assert_eq!(Charset::UTF_16LE, codec.charset());
   49|       |/// assert_eq!(
   50|       |///     Utf16::MAX_BYTES_PER_CHAR,
   51|       |///     <Utf16ByteCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
   52|       |/// );
   53|       |///
   54|       |/// let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
   55|       |/// let written = codec.encode_len(&'😀');
   56|       |/// unsafe {
   57|       |///     codec.encode(&'😀', &mut output, 0).expect("buffer fits");
   58|       |/// }
   59|       |/// let (value, consumed) = unsafe {
   60|       |///     codec.decode(&output[..written], 0).expect("valid UTF-16LE")
   61|       |/// };
   62|       |/// assert_eq!(('😀', written), (value, consumed.get()));
   63|       |/// ```
   64|       |#[derive(Clone, Copy, Debug, Eq, PartialEq)]
   65|       |pub struct Utf16ByteCodec {
   66|       |    /// Byte order used by both encoder and decoder paths.
   67|       |    byte_order: ByteOrder,
   68|       |}
   69|       |
   70|       |impl Utf16ByteCodec {
   71|       |    /// Creates a byte-serialized UTF-16 codec.
   72|       |    ///
   73|       |    /// # Parameters
   74|       |    ///
   75|       |    /// - `byte_order`: The byte order used by the byte buffer.
   76|       |    ///
   77|       |    /// # Returns
   78|       |    ///
   79|       |    /// Returns a UTF-16 byte codec.
   80|       |    #[inline]
   81|       |    #[must_use]
   82|    559|    pub const fn new(byte_order: ByteOrder) -> Self {
   83|    559|        Self { byte_order }
   84|    559|    }
   85|       |
   86|       |    /// Returns the configured byte order.
   87|       |    ///
   88|       |    /// # Returns
   89|       |    ///
   90|       |    /// Returns the byte order used by this codec.
   91|       |    #[inline]
   92|       |    #[must_use]
   93|      2|    pub const fn byte_order(self) -> ByteOrder {
   94|      2|        self.byte_order
   95|      2|    }
   96|       |
   97|       |    /// Returns the fixed-endian UTF-16 charset descriptor.
   98|       |    ///
   99|       |    /// # Returns
  100|       |    ///
  101|       |    /// Returns [`Charset::UTF_16LE`] or [`Charset::UTF_16BE`] according to this
  102|       |    /// codec's configured byte order.
  103|       |    #[inline]
  104|       |    #[must_use]
  105|      2|    pub const fn charset(self) -> Charset {
  106|      2|        Charset::from_utf16_byte_order(self.byte_order)
  107|      2|    }
  108|       |}
  109|       |
  110|       |impl CharsetCodec for Utf16ByteCodec {
  111|       |    /// Returns the fixed-endian UTF-16 charset for the configured byte order.
  112|       |    ///
  113|       |    /// # Returns
  114|       |    ///
  115|       |    /// Returns [`Charset::UTF_16BE`] when configured with
  116|       |    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_16LE`].
  117|       |    #[inline]
  118|    258|    fn charset(&self) -> Charset {
  119|    258|        Charset::from_utf16_byte_order(self.byte_order)
  120|    258|    }
  121|       |}
  122|       |
  123|       |impl Codec for Utf16ByteCodec {
  124|       |    type Value = char;
  125|       |    type Unit = u8;
  126|       |    type DecodeError = CharsetDecodeError;
  127|       |    type EncodeError = CharsetEncodeError;
  128|       |
  129|       |    const MIN_UNITS_PER_VALUE: usize = 2;
  130|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = Utf16::MAX_BYTES_PER_CHAR;
  131|       |
  132|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = Utf16::MAX_BYTES_PER_CHAR;
  133|       |
  134|       |    #[inline]
  135|  16.4k|    fn encode_len(&self, ch: &char) -> usize {
  136|  16.4k|        Utf16::unit_len(*ch) * 2
  137|  16.4k|    }
  138|       |
  139|       |    #[inline]
  140|  16.1k|    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
  141|  16.1k|        let (ch, consumed) =
  142|  16.1k|            decode_bytes_prefix(input, input_index, self.byte_order).map_err(CharsetDecodeError::into_codec_failure)?;
                                                                                                                                  ^16
  143|  16.1k|        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
  144|  16.1k|        Ok((ch, consumed))
  145|  16.1k|    }
  146|       |
  147|       |    #[inline]
  148|  16.1k|    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
  149|  16.1k|        let written = encode_bytes_char(*ch, output, self.byte_order, output_index);
  150|  16.1k|        debug_assert_eq!(written, ch.len_utf16() * 2);
  151|  16.1k|        debug_assert!(written <= output.len().saturating_sub(output_index));
  152|  16.1k|        Ok(written)
  153|  16.1k|    }
  154|       |}
  155|       |
  156|       |/// Decodes the first UTF-16 character from a closed byte buffer.
  157|       |///
  158|       |/// The input bytes are interpreted with `byte_order`, then decoded using UTF-16
  159|       |/// surrogate rules.
  160|       |///
  161|       |/// # Arguments
  162|       |///
  163|       |/// * `input` - UTF-16 encoded byte slice. Callers using closed-prefix decoding
  164|       |///   provide at least [`Utf16::MAX_BYTES_PER_CHAR`] readable bytes unless EOF
  165|       |///   has been reached.
  166|       |/// * `index` - Start offset in `input` bytes; must be `<= input.len()`.
  167|       |/// * `byte_order` - Byte order used to read UTF-16 units.
  168|       |///
  169|       |/// # Returns
  170|       |///
  171|       |/// Returns the decoded character and the non-zero number of consumed bytes.
  172|       |///
  173|       |/// # Errors
  174|       |///
  175|       |/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 byte
  176|       |///   sequences or malformed surrogate usage. Surrogates are valid UTF-16 code
  177|       |///   units only inside well-formed pairs, so malformed pair structure is
  178|       |///   reported as a sequence error rather than as an independently decoded
  179|       |///   scalar value.
  180|       |/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before a
  181|       |///   complete UTF-16 unit or surrogate pair is available.
  182|       |#[inline]
  183|  16.1k|fn decode_bytes_prefix(input: &[u8], index: usize, byte_order: ByteOrder) -> CharsetDecodeResult<(char, NonZeroUsize)> {
  184|  16.1k|    let charset = Charset::from_utf16_byte_order(byte_order);
  185|  16.1k|    debug_assert!(SliceRange::range_fits(input.len(), index, 2));
  186|  16.1k|    let available = input.len() - index;
  187|  16.1k|    let first = read_ordered_u16(input, index, byte_order);
  188|  16.1k|    if Utf16::is_high_surrogate(first) {
  189|  3.99k|        if available < 4 {
  190|      1|            let kind = CharsetDecodeErrorKind::IncompleteSequence { required: 4, available };
  191|      1|            return Err(CharsetDecodeError::new(charset, kind, index));
  192|  3.99k|        }
  193|  3.99k|        let second = read_ordered_u16(input, index + 2, byte_order);
  194|  3.99k|        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
  195|  3.98k|            Some(ch) => Ok((ch, nonzero(4))),
  196|       |            None => {
  197|      2|                let kind = CharsetDecodeErrorKind::malformed(second as u32);
  198|      2|                Err(CharsetDecodeError::new(charset, kind, index.saturating_add(2)).with_consumed(nonzero(2)))
  199|       |            }
  200|       |        }
  201|  12.1k|    } else if Utf16::is_low_surrogate(first) {
  202|     13|        let kind = CharsetDecodeErrorKind::malformed(first as u32);
  203|     13|        Err(CharsetDecodeError::new(charset, kind, index).with_consumed(nonzero(2)))
  204|       |    } else {
  205|  12.1k|        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
  206|  12.1k|        Ok((ch, nonzero(2)))
  207|       |    }
  208|  16.1k|}
  209|       |
  210|       |/// Encodes one character into byte-serialized UTF-16 at `index` in `output`.
  211|       |///
  212|       |/// # Arguments
  213|       |///
  214|       |/// * `ch` - The character to encode.
  215|       |/// * `output` - Byte destination.
  216|       |/// * `byte_order` - Byte order for writing UTF-16 units.
  217|       |/// * `index` - Start offset in `output` bytes; must be `<= output.len()`.
  218|       |///
  219|       |/// # Returns
  220|       |///
  221|       |/// `Ok(usize)` with the number of bytes written (`2` for BMP, `4` for
  222|       |/// supplementary).
  223|       |#[inline]
  224|  16.1k|fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> usize {
  225|  16.1k|    let required = Utf16::unit_len(ch) * 2;
  226|  16.1k|    debug_assert!(SliceRange::range_fits(output.len(), index, required));
  227|  16.1k|    let code_point = ch as u32;
  228|  16.1k|    if required == 2 {
  229|  12.1k|        write_ordered_u16(output, index, code_point as u16, byte_order);
  230|  12.1k|    } else {
  231|  4.00k|        let high = Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate");
  232|  4.00k|        let low = Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate");
  233|  4.00k|        write_ordered_u16(output, index, high, byte_order);
  234|  4.00k|        write_ordered_u16(output, index + 2, low, byte_order);
  235|  4.00k|    }
  236|  16.1k|    required
  237|  16.1k|}
  238|       |
  239|       |/// Reads one endian-aware `u16` value from an already checked byte slice.
  240|       |///
  241|       |/// # Parameters
  242|       |///
  243|       |/// - `input`: Source byte slice.
  244|       |/// - `index`: Start byte offset. The caller must guarantee two bytes are
  245|       |///   available from this offset.
  246|       |/// - `byte_order`: Byte order used to interpret the two bytes.
  247|       |///
  248|       |/// # Returns
  249|       |///
  250|       |/// Returns the decoded UTF-16 unit.
  251|       |#[inline]
  252|  20.1k|fn read_ordered_u16(input: &[u8], index: usize, byte_order: ByteOrder) -> u16 {
  253|       |    // SAFETY: The caller guarantees that two bytes are readable from `index`.
  254|  20.1k|    let bytes = unsafe {
  255|  20.1k|        [
  256|  20.1k|            UncheckedSlice::read(input, index),
  257|  20.1k|            UncheckedSlice::read(input, index + 1),
  258|  20.1k|        ]
  259|       |    };
  260|  20.1k|    match byte_order {
  261|    163|        ByteOrder::BigEndian => u16::from_be_bytes(bytes),
  262|  19.9k|        ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
  263|      0|        ByteOrder::NativeEndian => u16::from_ne_bytes(bytes),
  264|       |    }
  265|  20.1k|}
  266|       |
  267|       |/// Writes one endian-aware `u16` value into an already checked byte slice.
  268|       |///
  269|       |/// # Parameters
  270|       |///
  271|       |/// - `output`: Destination byte slice.
  272|       |/// - `index`: Start byte offset. The caller must guarantee two bytes are
  273|       |///   writable from this offset.
  274|       |/// - `unit`: UTF-16 unit to write.
  275|       |/// - `byte_order`: Byte order used to serialize the unit.
  276|       |#[inline]
  277|  20.1k|fn write_ordered_u16(output: &mut [u8], index: usize, unit: u16, byte_order: ByteOrder) {
  278|  20.1k|    let bytes = match byte_order {
  279|    180|        ByteOrder::BigEndian => unit.to_be_bytes(),
  280|  19.9k|        ByteOrder::LittleEndian => unit.to_le_bytes(),
  281|      0|        ByteOrder::NativeEndian => unit.to_ne_bytes(),
  282|       |    };
  283|       |    // SAFETY: The caller guarantees that two bytes are writable from `index`.
  284|  20.1k|    unsafe {
  285|  20.1k|        UncheckedSlice::write(output, index, bytes[0]);
  286|  20.1k|        UncheckedSlice::write(output, index + 1, bytes[1]);
  287|  20.1k|    }
  288|  20.1k|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/utf16_u16_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::Codec;
    9|       |use qubit_utils::SliceRange;
   10|       |use qubit_utils::UncheckedSlice;
   11|       |use qubit_utils::nonzero;
   12|       |
   13|       |use crate::Charset;
   14|       |use crate::CharsetCodec;
   15|       |use crate::CharsetDecodeError;
   16|       |use crate::CharsetDecodeErrorKind;
   17|       |use crate::CharsetDecodeResult;
   18|       |use crate::CharsetEncodeError;
   19|       |use crate::CharsetEncodeResult;
   20|       |use crate::Unicode;
   21|       |use crate::Utf16;
   22|       |use crate::error::CharsetCodecDecodeResult;
   23|       |
   24|       |/// Combined UTF-16 `u16` code-unit codec.
   25|       |///
   26|       |/// `Utf16U16Codec` works with UTF-16 code units rather than serialized bytes.
   27|       |/// Use [`crate::Utf16ByteCodec`] when the input or output is a byte stream with
   28|       |/// an explicit byte order.
   29|       |///
   30|       |/// # Invariant
   31|       |///
   32|       |/// `u16` units are interpreted as host `u16` values that already contain
   33|       |/// UTF-16 code units. This codec does not define a serialized byte order. Data
   34|       |/// stored in files, sent over a network, or exchanged across byte streams must
   35|       |/// be serialized with [`crate::Utf16ByteCodec`] first.
   36|       |///
   37|       |/// # Examples
   38|       |///
   39|       |/// ```rust
   40|       |/// use qubit_codec::Codec;
   41|       |/// use qubit_codec_text::{
   42|       |///     CharsetCodec,
   43|       |///     Charset,
   44|       |///     Utf16,
   45|       |///     Utf16U16Codec,
   46|       |/// };
   47|       |///
   48|       |/// let mut codec = Utf16U16Codec;
   49|       |/// assert_eq!(Charset::UTF_16, codec.charset());
   50|       |/// assert_eq!(
   51|       |///     Utf16::MAX_UNITS_PER_CHAR,
   52|       |///     <Utf16U16Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
   53|       |/// );
   54|       |///
   55|       |/// let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
   56|       |/// let written = codec.encode_len(&'😀');
   57|       |/// unsafe {
   58|       |///     codec.encode(&'😀', &mut output, 0).expect("buffer fits");
   59|       |/// }
   60|       |/// let (value, consumed) = unsafe {
   61|       |///     codec.decode(&output[..written], 0).expect("valid UTF-16")
   62|       |/// };
   63|       |/// assert_eq!(('😀', written), (value, consumed.get()));
   64|       |/// ```
   65|       |#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
   66|       |pub struct Utf16U16Codec;
   67|       |
   68|       |impl Utf16U16Codec {
   69|       |    /// Returns the UTF-16 encoding descriptor.
   70|       |    ///
   71|       |    /// # Returns
   72|       |    ///
   73|       |    /// Returns [`Charset::UTF_16`].
   74|       |    #[must_use]
   75|       |    #[inline]
   76|      3|    pub const fn charset(self) -> Charset {
   77|      3|        Charset::UTF_16
   78|      3|    }
   79|       |}
   80|       |
   81|       |impl CharsetCodec for Utf16U16Codec {
   82|       |    /// Returns UTF-16 charset descriptor.
   83|       |    ///
   84|       |    /// # Returns
   85|       |    ///
   86|       |    /// Returns [`Charset::UTF_16`].
   87|       |    #[inline]
   88|      3|    fn charset(&self) -> Charset {
   89|      3|        Charset::UTF_16
   90|      3|    }
   91|       |}
   92|       |
   93|       |impl Codec for Utf16U16Codec {
   94|       |    type Value = char;
   95|       |    type Unit = u16;
   96|       |    type DecodeError = CharsetDecodeError;
   97|       |    type EncodeError = CharsetEncodeError;
   98|       |
   99|       |    const MIN_UNITS_PER_VALUE: usize = 1;
  100|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = Utf16::MAX_UNITS_PER_CHAR;
  101|       |
  102|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = Utf16::MAX_UNITS_PER_CHAR;
  103|       |
  104|       |    #[inline]
  105|    204|    fn encode_len(&self, ch: &char) -> usize {
  106|    204|        Utf16::unit_len(*ch)
  107|    204|    }
  108|       |
  109|       |    #[inline]
  110|    140|    unsafe fn decode(
  111|    140|        &mut self,
  112|    140|        input: &[u16],
  113|    140|        input_index: usize,
  114|    140|    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
  115|    140|        let (ch, consumed) = decode_units_prefix(input, input_index).map_err(CharsetDecodeError::into_codec_failure)?;
                           ^126^126                                                                                               ^14
  116|    126|        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
  117|    126|        Ok((ch, consumed))
  118|    140|    }
  119|       |
  120|       |    #[inline]
  121|    174|    unsafe fn encode(&mut self, ch: &char, output: &mut [u16], output_index: usize) -> CharsetEncodeResult<usize> {
  122|    174|        let written = encode_units_char(*ch, output, output_index);
  123|    174|        debug_assert_eq!(written, ch.len_utf16());
  124|    174|        debug_assert!(written <= output.len().saturating_sub(output_index));
  125|    174|        Ok(written)
  126|    174|    }
  127|       |}
  128|       |
  129|       |/// Decodes the first UTF-16 character from a closed `u16` buffer.
  130|       |///
  131|       |/// The function handles three cases:
  132|       |/// 1. ASCII/non-surrogate units decode to a single `char`.
  133|       |/// 2. High-surrogate pairs are combined with the following unit into one scalar
  134|       |///    value.
  135|       |/// 3. Isolated low-surrogates are rejected as malformed.
  136|       |///
  137|       |/// # Arguments
  138|       |///
  139|       |/// * `input` - UTF-16 unit slice to decode from. Normal streaming callers
  140|       |///   provide at least [`Utf16::MAX_UNITS_PER_CHAR`] readable units unless EOF
  141|       |///   has been reached.
  142|       |/// * `index` - Start offset in `input`; must be `<= input.len()`.
  143|       |///
  144|       |/// # Returns
  145|       |///
  146|       |/// Returns the decoded character and the non-zero number of consumed UTF-16
  147|       |/// units.
  148|       |///
  149|       |/// # Errors
  150|       |///
  151|       |/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 code-unit
  152|       |///   structure, such as invalid high/low surrogate pairing. Surrogates are
  153|       |///   valid UTF-16 code units only inside well-formed pairs, so malformed pair
  154|       |///   structure is reported as a sequence error rather than as an independently
  155|       |///   decoded scalar value.
  156|       |/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before a
  157|       |///   complete surrogate pair is available.
  158|       |///
  159|       |/// # Panics
  160|       |///
  161|       |/// This function does not panic for invalid UTF-16 input because invalid input
  162|       |/// is surfaced as `CharsetDecodeError`.
  163|       |#[inline]
  164|    140|fn decode_units_prefix(input: &[u16], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
  165|    140|    debug_assert!(index < input.len());
  166|       |    // SAFETY: The caller guarantees that at least one unit is readable from
  167|       |    // `index`.
  168|    140|    let first = unsafe { UncheckedSlice::read(input, index) };
  169|    140|    if Utf16::is_high_surrogate(first) {
  170|     36|        if !SliceRange::range_fits(input.len(), index, 2) {
  171|      2|            let kind = CharsetDecodeErrorKind::IncompleteSequence {
  172|      2|                required: 2,
  173|      2|                available: input.len() - index,
  174|      2|            };
  175|      2|            return Err(CharsetDecodeError::new(Charset::UTF_16, kind, index));
  176|     34|        }
  177|     34|        let second = unit_at(input, index + 1);
  178|     34|        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
  179|     30|            Some(ch) => Ok((ch, nonzero(2))),
  180|       |            None => {
  181|      4|                let kind = CharsetDecodeErrorKind::malformed(second as u32);
  182|      4|                Err(CharsetDecodeError::new(Charset::UTF_16, kind, index.saturating_add(1))
  183|      4|                    .with_consumed(core::num::NonZeroUsize::MIN))
  184|       |            }
  185|       |        }
  186|    104|    } else if Utf16::is_low_surrogate(first) {
  187|      8|        let kind = CharsetDecodeErrorKind::malformed(first as u32);
  188|      8|        Err(CharsetDecodeError::new(Charset::UTF_16, kind, index))
  189|       |    } else {
  190|     96|        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
  191|     96|        Ok((ch, core::num::NonZeroUsize::MIN))
  192|       |    }
  193|    140|}
  194|       |
  195|       |/// Reads one UTF-16 unit from an already checked slice.
  196|       |#[inline]
  197|     34|fn unit_at(input: &[u16], index: usize) -> u16 {
  198|     34|    debug_assert!(index < input.len());
  199|       |    // SAFETY: Callers check sequence availability before reading the unit.
  200|     34|    unsafe { UncheckedSlice::read(input, index) }
  201|     34|}
  202|       |
  203|       |/// Encodes one character into UTF-16 `u16` units at `index` in `output`.
  204|       |///
  205|       |/// The helper returns how many UTF-16 units are written:
  206|       |/// one for BMP scalars, two for supplementary scalars.
  207|       |///
  208|       |/// # Arguments
  209|       |///
  210|       |/// * `ch` - The character to encode.
  211|       |/// * `output` - Destination unit buffer.
  212|       |/// * `index` - Start offset in `output`; must be `<= output.len()`.
  213|       |///
  214|       |/// # Returns
  215|       |///
  216|       |/// `Ok(usize)` with the number of written UTF-16 units (`1` or `2`).
  217|       |#[inline]
  218|    174|fn encode_units_char(ch: char, output: &mut [u16], index: usize) -> usize {
  219|    174|    let length = Utf16::unit_len(ch);
  220|    174|    debug_assert!(
  221|    174|        SliceRange::range_fits(output.len(), index, length),
  222|       |        "index + length exceeds output length"
  223|       |    );
  224|    174|    let code_point = ch as u32;
  225|       |    unsafe {
  226|       |        // SAFETY: The caller guarantees that `length` units are writable from
  227|       |        // `index`.
  228|    174|        if length == 1 {
  229|    137|            UncheckedSlice::write(output, index, code_point as u16);
  230|    137|        } else {
  231|     37|            UncheckedSlice::write(
  232|     37|                output,
  233|     37|                index,
  234|     37|                Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate"),
  235|     37|            );
  236|     37|            UncheckedSlice::write(
  237|     37|                output,
  238|     37|                index + 1,
  239|     37|                Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate"),
  240|     37|            );
  241|     37|        }
  242|       |    }
  243|    174|    length
  244|    174|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/utf32_byte_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::ByteOrder;
    9|       |use qubit_codec::Codec;
   10|       |use qubit_utils::SliceRange;
   11|       |use qubit_utils::UncheckedSlice;
   12|       |use qubit_utils::nonzero;
   13|       |
   14|       |use crate::Charset;
   15|       |use crate::CharsetCodec;
   16|       |use crate::CharsetDecodeError;
   17|       |use crate::CharsetDecodeErrorKind;
   18|       |use crate::CharsetDecodeResult;
   19|       |use crate::CharsetEncodeError;
   20|       |use crate::CharsetEncodeResult;
   21|       |use crate::Unicode;
   22|       |use crate::Utf32;
   23|       |use crate::error::CharsetCodecDecodeResult;
   24|       |
   25|       |/// Combined byte-serialized UTF-32 codec.
   26|       |///
   27|       |/// The codec uses one configured byte order for both decoding and encoding. It
   28|       |/// does not detect, consume, or emit a BOM automatically; callers should use
   29|       |/// [`crate::UnicodeBom`] when a byte stream may carry an explicit BOM.
   30|       |///
   31|       |/// # Examples
   32|       |///
   33|       |/// ```rust
   34|       |/// use qubit_codec::{
   35|       |///     ByteOrder,
   36|       |///     Codec,
   37|       |/// };
   38|       |/// use qubit_codec_text::{
   39|       |///     CharsetCodec,
   40|       |///     Charset,
   41|       |///     Utf32,
   42|       |///     Utf32ByteCodec,
   43|       |/// };
   44|       |///
   45|       |/// let mut codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
   46|       |/// assert_eq!(Charset::UTF_32BE, codec.charset());
   47|       |/// assert_eq!(
   48|       |///     Utf32::MAX_BYTES_PER_CHAR,
   49|       |///     <Utf32ByteCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
   50|       |/// );
   51|       |///
   52|       |/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
   53|       |/// let written = codec.encode_len(&'中');
   54|       |/// unsafe {
   55|       |///     codec.encode(&'中', &mut output, 0).expect("buffer fits");
   56|       |/// }
   57|       |/// let (value, consumed) = unsafe {
   58|       |///     codec.decode(&output[..written], 0).expect("valid UTF-32BE")
   59|       |/// };
   60|       |/// assert_eq!(('中', written), (value, consumed.get()));
   61|       |/// ```
   62|       |#[derive(Clone, Copy, Debug, Eq, PartialEq)]
   63|       |pub struct Utf32ByteCodec {
   64|       |    /// Byte order used by both encoder and decoder paths.
   65|       |    byte_order: ByteOrder,
   66|       |}
   67|       |
   68|       |impl Utf32ByteCodec {
   69|       |    /// Creates a byte-serialized UTF-32 codec.
   70|       |    ///
   71|       |    /// # Parameters
   72|       |    ///
   73|       |    /// - `byte_order`: The byte order used by the byte buffer.
   74|       |    ///
   75|       |    /// # Returns
   76|       |    ///
   77|       |    /// Returns a UTF-32 byte codec.
   78|       |    #[must_use]
   79|       |    #[inline]
   80|     45|    pub const fn new(byte_order: ByteOrder) -> Self {
   81|     45|        Self { byte_order }
   82|     45|    }
   83|       |
   84|       |    /// Returns the configured byte order.
   85|       |    ///
   86|       |    /// # Returns
   87|       |    ///
   88|       |    /// Returns the byte order used by this codec.
   89|       |    #[must_use]
   90|       |    #[inline]
   91|      2|    pub const fn byte_order(self) -> ByteOrder {
   92|      2|        self.byte_order
   93|      2|    }
   94|       |
   95|       |    /// Returns the fixed-endian UTF-32 charset descriptor.
   96|       |    ///
   97|       |    /// # Returns
   98|       |    ///
   99|       |    /// Returns [`Charset::UTF_32LE`] or [`Charset::UTF_32BE`] according to this
  100|       |    /// codec's configured byte order.
  101|       |    #[must_use]
  102|       |    #[inline]
  103|      2|    pub const fn charset(self) -> Charset {
  104|      2|        Charset::from_utf32_byte_order(self.byte_order)
  105|      2|    }
  106|       |}
  107|       |
  108|       |impl CharsetCodec for Utf32ByteCodec {
  109|       |    /// Returns the fixed-endian UTF-32 charset for the configured byte order.
  110|       |    ///
  111|       |    /// # Returns
  112|       |    ///
  113|       |    /// Returns [`Charset::UTF_32BE`] when configured with
  114|       |    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_32LE`].
  115|       |    #[inline]
  116|      2|    fn charset(&self) -> Charset {
  117|      2|        Charset::from_utf32_byte_order(self.byte_order)
  118|      2|    }
  119|       |}
  120|       |
  121|       |impl Codec for Utf32ByteCodec {
  122|       |    type Value = char;
  123|       |    type Unit = u8;
  124|       |    type DecodeError = CharsetDecodeError;
  125|       |    type EncodeError = CharsetEncodeError;
  126|       |
  127|       |    const MIN_UNITS_PER_VALUE: usize = 4;
  128|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = Utf32::MAX_BYTES_PER_CHAR;
  129|       |
  130|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = Utf32::MAX_BYTES_PER_CHAR;
  131|       |
  132|       |    #[inline]
  133|    258|    unsafe fn decode(
  134|    258|        &mut self,
  135|    258|        input: &[u8],
  136|    258|        input_index: usize,
  137|    258|    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
  138|    245|        let (ch, consumed) =
  139|    258|            decode_bytes_prefix(input, input_index, self.byte_order).map_err(CharsetDecodeError::into_codec_failure)?;
                                                                                                                                  ^13
  140|    245|        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
  141|    245|        Ok((ch, consumed))
  142|    258|    }
  143|       |
  144|       |    #[inline]
  145|    289|    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
  146|    289|        let written = encode_bytes_char(*ch, output, self.byte_order, output_index);
  147|    289|        debug_assert_eq!(written, Utf32::MAX_BYTES_PER_CHAR);
  148|    289|        debug_assert!(written <= output.len().saturating_sub(output_index));
  149|    289|        Ok(Utf32::MAX_BYTES_PER_CHAR)
  150|    289|    }
  151|       |}
  152|       |
  153|       |/// Decodes the first UTF-32 character from a closed byte buffer.
  154|       |///
  155|       |/// The input bytes are interpreted according to `byte_order`.
  156|       |///
  157|       |/// # Arguments
  158|       |///
  159|       |/// * `input` - UTF-32 encoded byte slice.
  160|       |/// * `index` - Start byte offset; must be `<= input.len()`.
  161|       |/// * `byte_order` - Byte order used to read a `u32` unit.
  162|       |///
  163|       |/// # Returns
  164|       |///
  165|       |/// Returns the decoded character and a non-zero count of `4` consumed bytes.
  166|       |///
  167|       |/// # Errors
  168|       |///
  169|       |/// * `CharsetDecodeErrorKind::InvalidCodePoint` when the decoded unit is not a
  170|       |///   valid scalar. UTF-32 stores a numeric scalar value directly, so surrogate
  171|       |///   code points and values above `0x10FFFF` are reported as invalid code
  172|       |///   points instead of malformed multi-unit structure.
  173|       |#[inline]
  174|    258|fn decode_bytes_prefix(
  175|    258|    input: &[u8],
  176|    258|    index: usize,
  177|    258|    byte_order: ByteOrder,
  178|    258|) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
  179|    258|    let charset = Charset::from_utf32_byte_order(byte_order);
  180|    258|    debug_assert!(SliceRange::range_fits(input.len(), index, 4));
  181|    258|    let unit = read_ordered_u32(input, index, byte_order);
  182|    258|    match Unicode::to_char(unit) {
  183|    245|        Some(ch) => Ok((ch, nonzero(4))),
  184|       |        None => {
  185|     13|            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
  186|     13|            Err(CharsetDecodeError::new(charset, kind, index).with_consumed(nonzero(4)))
  187|       |        }
  188|       |    }
  189|    258|}
  190|       |
  191|       |/// Encodes one character into byte-serialized UTF-32 at `index` in `output`.
  192|       |///
  193|       |/// # Arguments
  194|       |///
  195|       |/// * `ch` - The character to encode.
  196|       |/// * `output` - Destination byte buffer.
  197|       |/// * `byte_order` - Byte order used to write the 4-byte representation.
  198|       |/// * `index` - Start byte offset; must satisfy `index <= output.len()`.
  199|       |///
  200|       |/// # Returns
  201|       |///
  202|       |/// `Ok(4)` on success, because UTF-32 always occupies exactly four bytes.
  203|       |#[inline]
  204|    289|fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> usize {
  205|    289|    let required = Utf32::MAX_BYTES_PER_CHAR;
  206|    289|    debug_assert!(SliceRange::range_fits(output.len(), index, required));
  207|    289|    write_ordered_u32(output, index, ch as u32, byte_order);
  208|    289|    required
  209|    289|}
  210|       |
  211|       |/// Reads one endian-aware `u32` value from an already checked byte slice.
  212|       |///
  213|       |/// # Parameters
  214|       |///
  215|       |/// - `input`: Source byte slice.
  216|       |/// - `index`: Start byte offset. The caller must guarantee four bytes are
  217|       |///   available from this offset.
  218|       |/// - `byte_order`: Byte order used to interpret the four bytes.
  219|       |///
  220|       |/// # Returns
  221|       |///
  222|       |/// Returns the decoded UTF-32 unit.
  223|       |#[inline]
  224|    258|fn read_ordered_u32(input: &[u8], index: usize, byte_order: ByteOrder) -> u32 {
  225|       |    // SAFETY: The caller guarantees that four bytes are readable from `index`.
  226|    258|    let bytes = unsafe {
  227|    258|        [
  228|    258|            UncheckedSlice::read(input, index),
  229|    258|            UncheckedSlice::read(input, index + 1),
  230|    258|            UncheckedSlice::read(input, index + 2),
  231|    258|            UncheckedSlice::read(input, index + 3),
  232|    258|        ]
  233|       |    };
  234|    258|    match byte_order {
  235|    128|        ByteOrder::BigEndian => u32::from_be_bytes(bytes),
  236|    130|        ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
  237|      0|        ByteOrder::NativeEndian => u32::from_ne_bytes(bytes),
  238|       |    }
  239|    258|}
  240|       |
  241|       |/// Writes one endian-aware `u32` value into an already checked byte slice.
  242|       |///
  243|       |/// # Parameters
  244|       |///
  245|       |/// - `output`: Destination byte slice.
  246|       |/// - `index`: Start byte offset. The caller must guarantee four bytes are
  247|       |///   writable from this offset.
  248|       |/// - `unit`: UTF-32 unit to write.
  249|       |/// - `byte_order`: Byte order used to serialize the unit.
  250|       |#[inline]
  251|    289|fn write_ordered_u32(output: &mut [u8], index: usize, unit: u32, byte_order: ByteOrder) {
  252|    289|    let bytes = match byte_order {
  253|    144|        ByteOrder::BigEndian => unit.to_be_bytes(),
  254|    145|        ByteOrder::LittleEndian => unit.to_le_bytes(),
  255|      0|        ByteOrder::NativeEndian => unit.to_ne_bytes(),
  256|       |    };
  257|       |    // SAFETY: The caller guarantees that four bytes are writable from `index`.
  258|    289|    unsafe {
  259|    289|        UncheckedSlice::write(output, index, bytes[0]);
  260|    289|        UncheckedSlice::write(output, index + 1, bytes[1]);
  261|    289|        UncheckedSlice::write(output, index + 2, bytes[2]);
  262|    289|        UncheckedSlice::write(output, index + 3, bytes[3]);
  263|    289|    }
  264|    289|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/utf32_u32_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::Codec;
    9|       |use qubit_utils::UncheckedSlice;
   10|       |
   11|       |use crate::Charset;
   12|       |use crate::CharsetCodec;
   13|       |use crate::CharsetDecodeError;
   14|       |use crate::CharsetDecodeErrorKind;
   15|       |use crate::CharsetDecodeResult;
   16|       |use crate::CharsetEncodeError;
   17|       |use crate::CharsetEncodeResult;
   18|       |use crate::Unicode;
   19|       |use crate::Utf32;
   20|       |use crate::error::CharsetCodecDecodeResult;
   21|       |
   22|       |/// Combined UTF-32 `u32` code-unit codec.
   23|       |///
   24|       |/// `Utf32U32Codec` works with raw UTF-32 scalar-value units rather than
   25|       |/// serialized bytes. Use [`crate::Utf32ByteCodec`] for byte streams with an
   26|       |/// explicit byte order.
   27|       |///
   28|       |/// # Invariant
   29|       |///
   30|       |/// `u32` units are interpreted as host `u32` values that already contain
   31|       |/// UTF-32 scalar values. This codec does not define a serialized byte order.
   32|       |/// Data stored in files, sent over a network, or exchanged across byte streams
   33|       |/// must be serialized with [`crate::Utf32ByteCodec`] first.
   34|       |///
   35|       |/// # Examples
   36|       |///
   37|       |/// ```rust
   38|       |/// use qubit_codec::Codec;
   39|       |/// use qubit_codec_text::{
   40|       |///     CharsetCodec,
   41|       |///     Charset,
   42|       |///     Utf32,
   43|       |///     Utf32U32Codec,
   44|       |/// };
   45|       |///
   46|       |/// let mut codec = Utf32U32Codec;
   47|       |/// assert_eq!(Charset::UTF_32, codec.charset());
   48|       |/// assert_eq!(
   49|       |///     Utf32::MAX_UNITS_PER_CHAR,
   50|       |///     <Utf32U32Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
   51|       |/// );
   52|       |///
   53|       |/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
   54|       |/// let written = codec.encode_len(&'中');
   55|       |/// unsafe {
   56|       |///     codec.encode(&'中', &mut output, 0).expect("buffer fits");
   57|       |/// }
   58|       |/// let (value, consumed) = unsafe {
   59|       |///     codec.decode(&output[..written], 0).expect("valid UTF-32")
   60|       |/// };
   61|       |/// assert_eq!(('中', written), (value, consumed.get()));
   62|       |/// ```
   63|       |#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
   64|       |pub struct Utf32U32Codec;
   65|       |
   66|       |impl Utf32U32Codec {
   67|       |    /// Returns the UTF-32 encoding descriptor.
   68|       |    ///
   69|       |    /// # Returns
   70|       |    ///
   71|       |    /// Returns [`Charset::UTF_32`].
   72|       |    #[must_use]
   73|       |    #[inline]
   74|      2|    pub const fn charset(self) -> Charset {
   75|      2|        Charset::UTF_32
   76|      2|    }
   77|       |}
   78|       |
   79|       |impl CharsetCodec for Utf32U32Codec {
   80|       |    /// Returns UTF-32 charset descriptor.
   81|       |    ///
   82|       |    /// # Returns
   83|       |    ///
   84|       |    /// Returns [`Charset::UTF_32`].
   85|       |    #[inline]
   86|      2|    fn charset(&self) -> Charset {
   87|      2|        Charset::UTF_32
   88|      2|    }
   89|       |}
   90|       |
   91|       |impl Codec for Utf32U32Codec {
   92|       |    type Value = char;
   93|       |    type Unit = u32;
   94|       |    type DecodeError = CharsetDecodeError;
   95|       |    type EncodeError = CharsetEncodeError;
   96|       |
   97|       |    const MIN_UNITS_PER_VALUE: usize = 1;
   98|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;
   99|       |
  100|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;
  101|       |
  102|       |    #[inline]
  103|    137|    unsafe fn decode(
  104|    137|        &mut self,
  105|    137|        input: &[u32],
  106|    137|        input_index: usize,
  107|    137|    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
  108|    137|        let (ch, consumed) = decode_units_prefix(input, input_index).map_err(CharsetDecodeError::into_codec_failure)?;
                           ^125^125                                                                                               ^12
  109|    125|        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
  110|    125|        Ok((ch, consumed))
  111|    137|    }
  112|       |
  113|       |    #[inline]
  114|    146|    unsafe fn encode(&mut self, ch: &char, output: &mut [u32], output_index: usize) -> CharsetEncodeResult<usize> {
  115|    146|        let written = encode_units_char(*ch, output, output_index);
  116|    146|        debug_assert_eq!(written, Utf32::MAX_UNITS_PER_CHAR);
  117|    146|        debug_assert!(written <= output.len().saturating_sub(output_index));
  118|    146|        Ok(Utf32::MAX_UNITS_PER_CHAR)
  119|    146|    }
  120|       |}
  121|       |
  122|       |/// Decodes the first UTF-32 character from a closed `u32` buffer.
  123|       |///
  124|       |/// Each UTF-32 unit is interpreted as a Unicode scalar value directly.
  125|       |///
  126|       |/// # Arguments
  127|       |///
  128|       |/// * `input` - UTF-32 unit slice to decode from.
  129|       |/// * `index` - Start offset in `input`; must be `<= input.len()`.
  130|       |///
  131|       |/// # Returns
  132|       |///
  133|       |/// Returns the decoded character and `1` consumed unit.
  134|       |///
  135|       |/// # Errors
  136|       |///
  137|       |/// * `CharsetDecodeErrorKind::InvalidCodePoint` when `input[index]` is not a
  138|       |///   valid scalar.
  139|       |#[inline]
  140|    137|fn decode_units_prefix(input: &[u32], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
  141|    137|    debug_assert!(index < input.len());
  142|       |    // SAFETY: The caller guarantees that `index` is readable.
  143|    137|    let unit = unsafe { UncheckedSlice::read(input, index) };
  144|    137|    match Unicode::to_char(unit) {
  145|    125|        Some(ch) => Ok((ch, core::num::NonZeroUsize::MIN)),
  146|       |        None => {
  147|     12|            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
  148|     12|            Err(CharsetDecodeError::new(Charset::UTF_32, kind, index))
  149|       |        }
  150|       |    }
  151|    137|}
  152|       |
  153|       |/// Encodes one character into a UTF-32 `u32` unit at `index` in `output`.
  154|       |///
  155|       |/// # Arguments
  156|       |///
  157|       |/// * `ch` - The character to encode.
  158|       |/// * `output` - Destination unit buffer.
  159|       |/// * `index` - Start offset in `output`; must satisfy `index < output.len()`.
  160|       |///
  161|       |/// # Returns
  162|       |///
  163|       |/// Always `Ok(1)` to indicate one unit was written.
  164|       |#[inline]
  165|    146|fn encode_units_char(ch: char, output: &mut [u32], index: usize) -> usize {
  166|    146|    debug_assert!(index < output.len());
  167|       |    // SAFETY: The caller guarantees that one unit is writable at `index`.
  168|    146|    unsafe {
  169|    146|        UncheckedSlice::write(output, index, ch as u32);
  170|    146|    }
  171|    146|    1
  172|    146|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/codec/utf8_codec.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::num::NonZeroUsize;
    9|       |
   10|       |use qubit_codec::Codec;
   11|       |use qubit_utils::SliceRange;
   12|       |use qubit_utils::UncheckedSlice;
   13|       |use qubit_utils::nonzero;
   14|       |
   15|       |use crate::Charset;
   16|       |use crate::CharsetCodec;
   17|       |use crate::CharsetDecodeError;
   18|       |use crate::CharsetDecodeErrorKind;
   19|       |use crate::CharsetDecodeResult;
   20|       |use crate::CharsetEncodeError;
   21|       |use crate::CharsetEncodeResult;
   22|       |use crate::Unicode;
   23|       |use crate::Utf8;
   24|       |use crate::error::CharsetCodecDecodeResult;
   25|       |
   26|       |/// UTF-8 byte-buffer charset codec.
   27|       |///
   28|       |/// # Examples
   29|       |///
   30|       |/// ```rust
   31|       |/// use qubit_codec::Codec;
   32|       |/// use qubit_codec_text::{
   33|       |///     CharsetCodec,
   34|       |///     Charset,
   35|       |///     Utf8,
   36|       |///     Utf8Codec,
   37|       |/// };
   38|       |///
   39|       |/// let mut codec = Utf8Codec;
   40|       |/// assert_eq!(Charset::UTF_8, codec.charset());
   41|       |/// assert_eq!(
   42|       |///     Utf8::MAX_UNITS_PER_CHAR,
   43|       |///     <Utf8Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
   44|       |/// );
   45|       |///
   46|       |/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
   47|       |/// let written = codec.encode_len(&'é');
   48|       |/// unsafe {
   49|       |///     codec.encode(&'é', &mut output, 0).expect("buffer fits");
   50|       |/// }
   51|       |/// let (value, consumed) = unsafe {
   52|       |///     codec.decode(&output[..written], 0).expect("valid UTF-8")
   53|       |/// };
   54|       |/// assert_eq!(('é', written), (value, consumed.get()));
   55|       |/// ```
   56|       |#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
   57|       |pub struct Utf8Codec;
   58|       |
   59|       |impl Utf8Codec {
   60|       |    /// Returns the UTF-8 encoding descriptor.
   61|       |    ///
   62|       |    /// # Returns
   63|       |    ///
   64|       |    /// Returns [`Charset::UTF_8`].
   65|       |    #[must_use]
   66|       |    #[inline]
   67|      4|    pub const fn charset(self) -> Charset {
   68|      4|        Charset::UTF_8
   69|      4|    }
   70|       |}
   71|       |
   72|       |impl CharsetCodec for Utf8Codec {
   73|       |    /// Returns UTF-8 charset descriptor.
   74|       |    ///
   75|       |    /// # Returns
   76|       |    ///
   77|       |    /// Returns [`Charset::UTF_8`].
   78|       |    #[inline]
   79|    262|    fn charset(&self) -> Charset {
   80|    262|        Charset::UTF_8
   81|    262|    }
   82|       |}
   83|       |
   84|       |impl Codec for Utf8Codec {
   85|       |    type Value = char;
   86|       |    type Unit = u8;
   87|       |    type DecodeError = CharsetDecodeError;
   88|       |    type EncodeError = CharsetEncodeError;
   89|       |
   90|       |    const MIN_UNITS_PER_VALUE: usize = 1;
   91|       |    const MAX_ENCODE_UNITS_PER_VALUE: usize = Utf8::MAX_UNITS_PER_CHAR;
   92|       |
   93|       |    const MAX_DECODE_UNITS_PER_VALUE: usize = Utf8::MAX_UNITS_PER_CHAR;
   94|       |
   95|       |    #[inline]
   96|  15.6k|    fn encode_len(&self, ch: &char) -> usize {
   97|  15.6k|        Utf8::byte_len(*ch)
   98|  15.6k|    }
   99|       |
  100|       |    #[inline]
  101|  74.9k|    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
  102|  74.9k|        let (ch, consumed) = decode_prefix(input, input_index).map_err(CharsetDecodeError::into_codec_failure)?;
                           ^48.7k^48.7k                                                                                       ^26.2k
  103|  48.7k|        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
  104|  48.7k|        Ok((ch, consumed))
  105|  74.9k|    }
  106|       |
  107|       |    #[inline]
  108|  15.3k|    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
  109|  15.3k|        let written = encode_char(*ch, output, output_index);
  110|  15.3k|        debug_assert_eq!(written, Utf8::byte_len(*ch));
  111|  15.3k|        debug_assert!(written <= output.len().saturating_sub(output_index));
  112|  15.3k|        Ok(written)
  113|  15.3k|    }
  114|       |}
  115|       |
  116|       |/// Decodes the first UTF-8 character from a closed byte slice starting at
  117|       |/// `index`.
  118|       |///
  119|       |/// The caller normally provides at least `Utf8::MAX_UNITS_PER_CHAR` readable
  120|       |/// bytes from `index`. If fewer bytes are present, this function treats the
  121|       |/// slice as closed at EOF: complete shorter UTF-8 sequences still decode, while
  122|       |/// truncated sequences return [`CharsetDecodeErrorKind::IncompleteSequence`].
  123|       |///
  124|       |/// # Arguments
  125|       |///
  126|       |/// * `input` - UTF-8 byte slice to decode from.
  127|       |/// * `index` - Start offset in `input`; must be `<= input.len()`.
  128|       |///
  129|       |/// # Returns
  130|       |///
  131|       |/// Returns the decoded character and the non-zero number of consumed bytes.
  132|       |///
  133|       |/// # Errors
  134|       |///
  135|       |/// * `CharsetDecodeErrorKind::MalformedSequence` when the first byte or
  136|       |///   continuation bytes are invalid for UTF-8.
  137|       |/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before the
  138|       |///   complete UTF-8 sequence is available.
  139|       |#[inline]
  140|  74.9k|fn decode_prefix(input: &[u8], index: usize) -> CharsetDecodeResult<(char, NonZeroUsize)> {
  141|  74.9k|    debug_assert!(index < input.len());
  142|       |    // SAFETY: The caller guarantees that at least one byte is readable from
  143|       |    // `index`.
  144|  74.9k|    let first = unsafe { UncheckedSlice::read(input, index) };
  145|  74.9k|    let length = match Utf8::byte_len_from_leading_byte(first) {
                      ^59.0k
  146|  59.0k|        Some(length) => length,
  147|       |        None => {
  148|  15.9k|            let kind = CharsetDecodeErrorKind::malformed(first as u32);
  149|  15.9k|            return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
  150|       |        }
  151|       |    };
  152|  59.0k|    if !SliceRange::range_fits(input.len(), index, length) {
  153|    114|        validate_partial(input, index)?;
                                                    ^25
  154|     89|        let kind = CharsetDecodeErrorKind::IncompleteSequence {
  155|     89|            required: length,
  156|     89|            available: input.len() - index,
  157|     89|        };
  158|     89|        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
  159|  58.9k|    }
  160|  58.9k|    let code_point = match length {
                      ^48.7k
  161|  40.4k|        1 => first as u32,
  162|  8.53k|        2 => decode_two(input, index)?,
                                                   ^5.41k
  163|  4.84k|        3 => decode_three(input, index)?,
                                                     ^3.59k
  164|  5.07k|        4 => decode_four(input, index)?,
                                                    ^1.17k
  165|      0|        _ => unreachable!("UTF-8 sequence length is limited to four bytes"),
  166|       |    };
  167|  48.7k|    let ch = Unicode::to_char(code_point).expect("well-formed UTF-8 decodes to a Unicode scalar");
  168|  48.7k|    Ok((ch, nonzero(length)))
  169|  74.9k|}
  170|       |
  171|       |/// Encodes one Unicode scalar value into UTF-8 at `index` in `output`.
  172|       |///
  173|       |/// The function writes the byte sequence for `ch` and returns how many bytes
  174|       |/// were written.
  175|       |///
  176|       |/// # Arguments
  177|       |///
  178|       |/// * `ch` - The character to encode.
  179|       |/// * `output` - Destination buffer.
  180|       |/// * `index` - Start offset in `output`; must satisfy `index <= output.len()`.
  181|       |///
  182|       |/// # Returns
  183|       |///
  184|       |/// `Ok(usize)` with the number of UTF-8 bytes written (`1..=4`).
  185|       |#[inline]
  186|  15.3k|fn encode_char(ch: char, output: &mut [u8], index: usize) -> usize {
  187|  15.3k|    let length = Utf8::byte_len(ch);
  188|       |    // SAFETY: The caller guarantees that `length` bytes are writable from
  189|       |    // `index`; `encode_utf8` writes directly into that checked range.
  190|  15.3k|    let target = unsafe { UncheckedSlice::subslice_mut(output, index, length) };
  191|  15.3k|    ch.encode_utf8(target);
  192|  15.3k|    length
  193|  15.3k|}
  194|       |
  195|       |/// Decodes a two-byte UTF-8 sequence starting at `index`.
  196|       |///
  197|       |/// # Arguments
  198|       |///
  199|       |/// * `input` - Byte slice containing the sequence.
  200|       |/// * `index` - Start offset of a two-byte leading byte.
  201|       |///
  202|       |/// # Returns
  203|       |///
  204|       |/// The decoded Unicode scalar value as a `u32` on success.
  205|       |///
  206|       |/// # Errors
  207|       |///
  208|       |/// * `CharsetDecodeErrorKind::MalformedSequence` when the second byte is not a
  209|       |///   valid UTF-8 continuation byte.
  210|       |#[inline]
  211|  8.53k|fn decode_two(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
  212|  8.53k|    let first = byte_at(input, index);
  213|  8.53k|    let second = validate_second_byte(input, index)?;
                      ^3.12k                                     ^5.41k
  214|  3.12k|    Ok((((first & 0x1f) as u32) << 6) | ((second & 0x3f) as u32))
  215|  8.53k|}
  216|       |
  217|       |/// Validates the bytes already present in an incomplete UTF-8 prefix.
  218|       |///
  219|       |/// This is used after the total sequence length is known, to catch malformed
  220|       |/// continuation bytes before more data arrives.
  221|       |///
  222|       |/// # Arguments
  223|       |///
  224|       |/// * `input` - Prefix slice being decoded.
  225|       |/// * `index` - Start offset of the current UTF-8 sequence.
  226|       |///
  227|       |/// # Returns
  228|       |///
  229|       |/// `Ok(())` if currently available bytes are structurally valid, otherwise a
  230|       |/// decoding error describing the first malformed position.
  231|       |#[inline]
  232|    114|fn validate_partial(input: &[u8], index: usize) -> CharsetDecodeResult<()> {
  233|    114|    if SliceRange::range_fits(input.len(), index, 2) {
  234|     39|        validate_second_byte(input, index)?;
                                                        ^21
  235|     75|    }
  236|     93|    if SliceRange::range_fits(input.len(), index, 3) {
  237|      5|        validate_continuation_byte(input, index, 2)?;
                                                                 ^4
  238|     88|    }
  239|     89|    Ok(())
  240|    114|}
  241|       |
  242|       |/// Checks whether `second` is legal for a UTF-8 leading byte `first`.
  243|       |///
  244|       |/// # Arguments
  245|       |///
  246|       |/// * `first` - UTF-8 leading byte.
  247|       |/// * `second` - Byte to validate as the first continuation-like byte.
  248|       |///
  249|       |/// # Returns
  250|       |///
  251|       |/// `true` when the pair `(first, second)` is valid for UTF-8 sequence decoding,
  252|       |/// otherwise `false`.
  253|       |#[inline]
  254|  18.4k|fn is_valid_second_byte(first: u8, second: u8) -> bool {
  255|  18.4k|    match first {
  256|  18.4k|        0xc2..=0xdf | 0xe1..=0xec | 0xee..=0xef | 0xf1..=0xf3 => Utf8::is_continuation_byte(second),
                                    ^9.69k        ^6.11k        ^3.85k         ^16.2k                     ^16.2k
  257|    271|        0xe0 => (0xa0..=0xbf).contains(&second),
  258|    247|        0xed => (0x80..=0x9f).contains(&second),
  259|  1.22k|        0xf0 => (0x90..=0xbf).contains(&second),
  260|    454|        0xf4 => (0x80..=0x8f).contains(&second),
  261|      0|        _ => false,
  262|       |    }
  263|  18.4k|}
  264|       |
  265|       |/// Decodes a three-byte UTF-8 sequence starting at `index`.
  266|       |///
  267|       |/// # Arguments
  268|       |///
  269|       |/// * `input` - Byte slice containing the sequence.
  270|       |/// * `index` - Start offset of a three-byte leading byte.
  271|       |///
  272|       |/// # Returns
  273|       |///
  274|       |/// The decoded Unicode scalar value as a `u32` on success.
  275|       |///
  276|       |/// # Errors
  277|       |///
  278|       |/// * `CharsetDecodeErrorKind::MalformedSequence` when the second or third byte
  279|       |///   is invalid.
  280|       |#[inline]
  281|  4.84k|fn decode_three(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
  282|  4.84k|    let first = byte_at(input, index);
  283|  4.84k|    let second = validate_second_byte(input, index)?;
                      ^1.94k                                     ^2.90k
  284|  1.94k|    let third = validate_continuation_byte(input, index, 2)?;
                      ^1.24k                                             ^698
  285|  1.24k|    Ok((((first & 0x0f) as u32) << 12) | (((second & 0x3f) as u32) << 6) | ((third & 0x3f) as u32))
  286|  4.84k|}
  287|       |
  288|       |/// Decodes a four-byte UTF-8 sequence starting at `index`.
  289|       |///
  290|       |/// # Arguments
  291|       |///
  292|       |/// * `input` - Byte slice containing the sequence.
  293|       |/// * `index` - Start offset of a four-byte leading byte.
  294|       |///
  295|       |/// # Returns
  296|       |///
  297|       |/// The decoded Unicode scalar value as a `u32` on success.
  298|       |///
  299|       |/// # Errors
  300|       |///
  301|       |/// * `CharsetDecodeErrorKind::MalformedSequence` when any continuation byte is
  302|       |///   invalid.
  303|       |#[inline]
  304|  5.07k|fn decode_four(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
  305|  5.07k|    let first = byte_at(input, index);
  306|  5.07k|    let second = validate_second_byte(input, index)?;
                      ^4.10k                                     ^966
  307|  4.10k|    let third = validate_continuation_byte(input, index, 2)?;
                      ^3.94k                                             ^167
  308|  3.94k|    let fourth = validate_continuation_byte(input, index, 3)?;
                      ^3.89k                                              ^43
  309|  3.89k|    Ok((((first & 0x07) as u32) << 18)
  310|  3.89k|        | (((second & 0x3f) as u32) << 12)
  311|  3.89k|        | (((third & 0x3f) as u32) << 6)
  312|  3.89k|        | ((fourth & 0x3f) as u32))
  313|  5.07k|}
  314|       |
  315|       |/// Validates the first continuation-like byte after a UTF-8 leading byte.
  316|       |#[inline]
  317|  18.4k|fn validate_second_byte(input: &[u8], index: usize) -> CharsetDecodeResult<u8> {
  318|  18.4k|    let first = byte_at(input, index);
  319|  18.4k|    let second = byte_at(input, index + 1);
  320|  18.4k|    if is_valid_second_byte(first, second) {
  321|  9.19k|        Ok(second)
  322|       |    } else {
  323|  9.30k|        Err(malformed_byte_error(second, index.saturating_add(1), nonzero(1)))
  324|       |    }
  325|  18.4k|}
  326|       |
  327|       |/// Validates a regular UTF-8 continuation byte at `offset` from `index`.
  328|       |#[inline]
  329|  10.0k|fn validate_continuation_byte(input: &[u8], index: usize, offset: usize) -> CharsetDecodeResult<u8> {
  330|  10.0k|    let byte = byte_at(input, index + offset);
  331|  10.0k|    if Utf8::is_continuation_byte(byte) {
  332|  9.08k|        Ok(byte)
  333|       |    } else {
  334|    912|        Err(malformed_byte_error(
  335|    912|            byte,
  336|    912|            index.saturating_add(offset),
  337|    912|            NonZeroUsize::new(offset).expect("UTF-8 consumed width is non-zero"),
  338|    912|        ))
  339|       |    }
  340|  10.0k|}
  341|       |
  342|       |/// Creates a malformed-byte error with consumed width metadata.
  343|  10.2k|fn malformed_byte_error(byte: u8, index: usize, consumed: NonZeroUsize) -> CharsetDecodeError {
  344|  10.2k|    let kind = CharsetDecodeErrorKind::malformed(byte as u32);
  345|  10.2k|    CharsetDecodeError::new(Charset::UTF_8, kind, index).with_consumed(consumed)
  346|  10.2k|}
  347|       |
  348|       |/// Reads one byte from an already checked slice.
  349|       |#[inline]
  350|  65.4k|fn byte_at(input: &[u8], index: usize) -> u8 {
  351|  65.4k|    debug_assert!(index < input.len());
  352|       |    // SAFETY: Callers check sequence availability before reading the byte.
  353|  65.4k|    unsafe { UncheckedSlice::read(input, index) }
  354|  65.4k|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/convert/charset_convert_error.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::TranscodeConvertError;
    9|       |use qubit_codec::TranscodeConvertErrorOf;
   10|       |use qubit_codec::TranscodeFailure;
   11|       |
   12|       |use crate::Charset;
   13|       |use crate::CharsetCodec;
   14|       |use crate::CharsetDecodeError;
   15|       |use crate::CharsetEncodeError;
   16|       |
   17|       |/// Error reported while converting between two charsets.
   18|       |#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
   19|       |#[non_exhaustive]
   20|       |pub enum CharsetConvertError {
   21|       |    /// Source decoding failed.
   22|       |    #[error("Failed to decode source charset: {0}")]
   23|       |    Decode(#[from] CharsetDecodeError),
   24|       |
   25|       |    /// Target encoding failed.
   26|       |    #[error("Failed to encode target charset: {0}")]
   27|       |    Encode(#[from] CharsetEncodeError),
   28|       |}
   29|       |
   30|       |impl CharsetConvertError {
   31|       |    /// Maps a low-level converter error into a charset-specific error.
   32|       |    ///
   33|       |    /// # Parameters
   34|       |    ///
   35|       |    /// - `source_charset`: Charset attached to source-side framework failures.
   36|       |    /// - `target_charset`: Charset attached to target-side framework failures.
   37|       |    /// - `error`: Low-level converter error to map.
   38|       |    ///
   39|       |    /// # Returns
   40|       |    ///
   41|       |    /// Returns a source [`Self::Decode`] error for source failures and a target
   42|       |    /// [`Self::Encode`] error for target failures. Domain errors retain their
   43|       |    /// original charset context.
   44|       |    #[must_use]
   45|     15|    pub fn from_transcode_error<D, E>(
   46|     15|        source_charset: Charset,
   47|     15|        target_charset: Charset,
   48|     15|        error: TranscodeConvertErrorOf<D, E>,
   49|     15|    ) -> Self
   50|     15|    where
   51|     15|        D: CharsetCodec,
   52|     15|        E: CharsetCodec,
   53|       |    {
   54|     15|        match error {
   55|     12|            TranscodeConvertError::Failure(failure) => {
   56|     12|                Self::from_transcode_failure(source_charset, target_charset, failure)
   57|       |            }
   58|      1|            TranscodeConvertError::DecodeDomain(error) => Self::Decode(error.into_source()),
   59|      1|            TranscodeConvertError::EncodeDomain(error) => Self::Encode(error.into_source()),
   60|      1|            TranscodeConvertError::Unencodable { input_index, value } => {
   61|      1|                Self::Encode(CharsetEncodeError::map_unencodable(target_charset, input_index, value))
   62|       |            }
   63|       |        }
   64|     15|    }
   65|       |
   66|       |    /// Maps a framework-level converter failure into a charset-specific error.
   67|       |    ///
   68|       |    /// # Parameters
   69|       |    ///
   70|       |    /// - `source_charset`: Charset attached to source-side failures.
   71|       |    /// - `target_charset`: Charset attached to target-side failures.
   72|       |    /// - `failure`: Framework-level converter failure to map.
   73|       |    ///
   74|       |    /// # Returns
   75|       |    ///
   76|       |    /// Returns [`Self::Decode`] for source-input failures and [`Self::Encode`]
   77|       |    /// for target-output failures. Other framework failures remain decode-side
   78|       |    /// unexpected failures to preserve the prior error contract.
   79|       |    #[must_use]
   80|     12|    fn from_transcode_failure(source_charset: Charset, target_charset: Charset, failure: TranscodeFailure) -> Self {
   81|     12|        match failure {
   82|      3|            failure @ (TranscodeFailure::InvalidInputIndex { .. }
   83|       |            | TranscodeFailure::IncompleteInput { .. }
   84|       |            | TranscodeFailure::TrailingInput { .. }) => {
   85|      3|                Self::Decode(CharsetDecodeError::map_transcode_failure(source_charset, failure))
   86|       |            }
   87|      6|            failure @ (TranscodeFailure::InvalidOutputIndex { .. }
   88|       |            | TranscodeFailure::InvalidOutputRange { .. }
   89|       |            | TranscodeFailure::InsufficientOutput { .. }
   90|       |            | TranscodeFailure::OutputLengthOverflow) => {
   91|      6|                Self::Encode(CharsetEncodeError::map_transcode_failure(target_charset, failure))
   92|       |            }
   93|      3|            failure => Self::Decode(CharsetDecodeError::map_transcode_failure(source_charset, failure)),
   94|       |        }
   95|     12|    }
   96|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/convert/charset_converter.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::CapacityError;
    9|       |use qubit_codec::TranscodeConvertErrorOf;
   10|       |use qubit_codec::TranscodeConverter;
   11|       |use qubit_codec::TranscodeProgress;
   12|       |use qubit_codec::Transcoder;
   13|       |use qubit_codec::engine::TranscodeConvertEngine;
   14|       |
   15|       |use crate::CharsetCodec;
   16|       |use crate::CharsetConvertError;
   17|       |use crate::CharsetDecodeError;
   18|       |use crate::CharsetDecodeHooks;
   19|       |use crate::CharsetDecodePolicy;
   20|       |use crate::CharsetEncodeError;
   21|       |use crate::CharsetEncodeHooks;
   22|       |use crate::CharsetEncodePolicy;
   23|       |use crate::CharsetEncoder;
   24|       |use crate::MalformedAction;
   25|       |use crate::UnmappableAction;
   26|       |
   27|       |/// Converts units encoded with one charset into units encoded with another
   28|       |/// charset.
   29|       |///
   30|       |/// The converter owns the source and target charset codecs plus the same
   31|       |/// decode/encode policy hooks used by [`crate::CharsetDecoder`] and
   32|       |/// [`crate::CharsetEncoder`].
   33|       |/// A decoded character may be kept pending inside the common buffered convert
   34|       |/// engine when the target output buffer is full. During
   35|       |/// [`Transcoder::finish`], the converter drains internally retained
   36|       |/// output and finishes the composed decode/encode policy hooks. Callers remain
   37|       |/// responsible for passing any incomplete input tail to
   38|       |/// [`Transcoder::transcode_eof`] before finishing the logical stream.
   39|       |///
   40|       |/// # Type Parameters
   41|       |///
   42|       |/// - `D`: Low-level charset codec used by the source decoder.
   43|       |/// - `E`: Low-level charset codec used by the target encoder.
   44|       |///
   45|       |/// ```rust
   46|       |/// use qubit_codec::{
   47|       |///     TranscodeStatus,
   48|       |///     Transcoder,
   49|       |/// };
   50|       |/// use qubit_codec_text::{
   51|       |///     CharsetConverter,
   52|       |///     CharsetDecoder,
   53|       |///     CharsetEncoder,
   54|       |///     Utf16U16Codec,
   55|       |///     Utf8Codec,
   56|       |/// };
   57|       |///
   58|       |/// let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
   59|       |/// let mut output = [0_u16; 2];
   60|       |/// converter
   61|       |///     .reset(&mut [], 0)
   62|       |///     .expect("reset converter before streaming");
   63|       |///
   64|       |/// let progress = converter
   65|       |///     .transcode("AB".as_bytes(), 0, &mut output, 0)
   66|       |///     .expect("transcode bytes to utf-16");
   67|       |///
   68|       |/// assert_eq!(TranscodeStatus::Complete, progress.status());
   69|       |/// assert_eq!(2, progress.read());
   70|       |/// assert_eq!(2, progress.written());
   71|       |/// assert_eq!([65, 66], output);
   72|       |/// ```
   73|       |#[derive(Debug)]
   74|       |pub struct CharsetConverter<D, E>
   75|       |where
   76|       |    D: CharsetCodec,
   77|       |    E: CharsetCodec,
   78|       |{
   79|       |    /// Common buffered converter engine.
   80|       |    engine: TranscodeConvertEngine<D, E, CharsetDecodeHooks, CharsetEncodeHooks>,
   81|       |    /// Public malformed-input policy metadata.
   82|       |    decode_policy: CharsetDecodePolicy,
   83|       |    /// Public unmappable-input policy metadata.
   84|       |    encode_policy: CharsetEncodePolicy,
   85|       |}
   86|       |
   87|       |impl<D, E> CharsetConverter<D, E>
   88|       |where
   89|       |    D: CharsetCodec,
   90|       |    E: CharsetCodec,
   91|       |{
   92|       |    /// Creates a charset converter from raw source and target codecs.
   93|       |    ///
   94|       |    /// # Parameters
   95|       |    ///
   96|       |    /// - `source`: Source charset codec.
   97|       |    /// - `target`: Target charset codec.
   98|       |    ///
   99|       |    /// # Returns
  100|       |    ///
  101|       |    /// Returns a converter with the default decoder policy and the target
  102|       |    /// encoder policy that can be represented by `target`. The encoder policy
  103|       |    /// first tries [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`] and falls back
  104|       |    /// to [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] when needed.
  105|       |    ///
  106|       |    /// # Panics
  107|       |    ///
  108|       |    /// Panics when neither default replacement can be encoded by `target`.
  109|       |    /// This panic is intentional by design: reaching this branch means the
  110|       |    /// target codec implementation is wrong, because the API requires a
  111|       |    /// fallback replacement that the codec can encode. Built-in codecs can
  112|       |    /// always encode the fallback `?`; custom [`crate::CharsetCodec`]
  113|       |    /// implementations that cannot encode it violate the replacement
  114|       |    /// invariant and must fail fast during converter construction rather than
  115|       |    /// defer the bug to user input.
  116|       |    #[must_use]
  117|     23|    pub fn from_codecs(source: D, target: E) -> Self {
  118|     23|        let decode_policy = CharsetDecodePolicy::default();
  119|     23|        let (encode_policy, encode_hooks) = Self::default_encode_policy(&target);
  120|     23|        Self {
  121|     23|            engine: TranscodeConvertEngine::new(
  122|     23|                source,
  123|     23|                target,
  124|     23|                CharsetDecodeHooks::new(decode_policy.malformed_action(), decode_policy.replacement()),
  125|     23|                encode_hooks,
  126|     23|            ),
  127|     23|            decode_policy,
  128|     23|            encode_policy,
  129|     23|        }
  130|     23|    }
  131|       |
  132|       |    /// Creates a charset converter from raw codecs and explicit policies.
  133|       |    ///
  134|       |    /// # Parameters
  135|       |    ///
  136|       |    /// - `source`: Source charset codec.
  137|       |    /// - `target`: Target charset codec.
  138|       |    /// - `decode_policy`: Malformed source-input policy.
  139|       |    /// - `encode_policy`: Unmappable target-output policy.
  140|       |    ///
  141|       |    /// # Returns
  142|       |    ///
  143|       |    /// Returns a converter configured with the supplied policies.
  144|       |    ///
  145|       |    /// # Errors
  146|       |    ///
  147|       |    /// Returns an error when `encode_policy` uses replacement and the target
  148|       |    /// codec cannot encode the replacement character.
  149|     83|    pub fn from_codecs_with_policies(
  150|     83|        source: D,
  151|     83|        target: E,
  152|     83|        decode_policy: CharsetDecodePolicy,
  153|     83|        encode_policy: CharsetEncodePolicy,
  154|     83|    ) -> Result<Self, CharsetEncodeError> {
  155|     83|        let encode_hooks = CharsetEncoder::<E>::create_hooks(&target, encode_policy)?;
                          ^82                                                                     ^1
  156|     82|        Ok(Self {
  157|     82|            engine: TranscodeConvertEngine::new(
  158|     82|                source,
  159|     82|                target,
  160|     82|                CharsetDecodeHooks::new(decode_policy.malformed_action(), decode_policy.replacement()),
  161|     82|                encode_hooks,
  162|     82|            ),
  163|     82|            decode_policy,
  164|     82|            encode_policy,
  165|     82|        })
  166|     83|    }
  167|       |
  168|       |    /// Returns the charset decoded from source units.
  169|       |    ///
  170|       |    /// # Returns
  171|       |    ///
  172|       |    /// Returns the charset reported by the source codec.
  173|       |    #[inline]
  174|       |    #[must_use]
  175|      2|    pub fn source_charset(&self) -> crate::Charset {
  176|      2|        self.source_codec().charset()
  177|      2|    }
  178|       |
  179|       |    /// Returns the charset encoded to target units.
  180|       |    ///
  181|       |    /// # Returns
  182|       |    ///
  183|       |    /// Returns the charset reported by the target codec.
  184|       |    #[inline]
  185|       |    #[must_use]
  186|      2|    pub fn target_charset(&self) -> crate::Charset {
  187|      2|        self.target_codec().charset()
  188|      2|    }
  189|       |
  190|       |    /// Returns the source codec.
  191|       |    ///
  192|       |    /// # Returns
  193|       |    ///
  194|       |    /// Returns a shared reference to the source codec owned by this converter.
  195|       |    #[inline]
  196|       |    #[must_use]
  197|      3|    pub fn source_codec(&self) -> &D {
  198|      3|        self.engine.source_codec()
  199|      3|    }
  200|       |
  201|       |    /// Returns the source codec mutably.
  202|       |    ///
  203|       |    /// Mutating source codec configuration while a conversion is active can
  204|       |    /// invalidate buffered decode state. Reset the converter before continuing
  205|       |    /// with the modified codec.
  206|       |    ///
  207|       |    /// # Returns
  208|       |    ///
  209|       |    /// Returns a mutable reference to the source codec owned by this converter.
  210|       |    #[inline]
  211|       |    #[must_use]
  212|      1|    pub fn source_codec_mut(&mut self) -> &mut D {
  213|      1|        self.engine.source_codec_mut()
  214|      1|    }
  215|       |
  216|       |    /// Returns the target codec.
  217|       |    ///
  218|       |    /// # Returns
  219|       |    ///
  220|       |    /// Returns a shared reference to the target codec owned by this converter.
  221|       |    #[inline]
  222|       |    #[must_use]
  223|      3|    pub fn target_codec(&self) -> &E {
  224|      3|        self.engine.target_codec()
  225|      3|    }
  226|       |
  227|       |    /// Returns the target codec mutably.
  228|       |    ///
  229|       |    /// Mutating target codec configuration while a conversion is active can
  230|       |    /// invalidate buffered encode state and replacement validation assumptions.
  231|       |    /// Reset the converter before continuing with the modified codec.
  232|       |    ///
  233|       |    /// # Returns
  234|       |    ///
  235|       |    /// Returns a mutable reference to the target codec owned by this converter.
  236|       |    #[inline]
  237|       |    #[must_use]
  238|      1|    pub fn target_codec_mut(&mut self) -> &mut E {
  239|      1|        self.engine.target_codec_mut()
  240|      1|    }
  241|       |
  242|       |    /// Consumes the converter and returns its codecs.
  243|       |    ///
  244|       |    /// Converter policies, hook state, and pending conversion state are
  245|       |    /// discarded.
  246|       |    ///
  247|       |    /// # Returns
  248|       |    ///
  249|       |    /// Returns the source codec followed by the target codec.
  250|       |    #[inline]
  251|       |    #[must_use]
  252|      1|    pub fn into_codecs(self) -> (D, E) {
  253|      1|        let (source, target, _, _) = self.engine.into_parts();
  254|      1|        (source, target)
  255|      1|    }
  256|       |
  257|       |    /// Returns the configured malformed source-input policy.
  258|       |    ///
  259|       |    /// # Returns
  260|       |    ///
  261|       |    /// Returns the decoder policy used by this converter.
  262|       |    #[inline]
  263|       |    #[must_use]
  264|      3|    pub const fn decode_policy(&self) -> CharsetDecodePolicy {
  265|      3|        self.decode_policy
  266|      3|    }
  267|       |
  268|       |    /// Returns the configured unmappable target-output policy.
  269|       |    ///
  270|       |    /// # Returns
  271|       |    ///
  272|       |    /// Returns the encoder policy used by this converter.
  273|       |    #[inline]
  274|       |    #[must_use]
  275|      2|    pub const fn encode_policy(&self) -> CharsetEncodePolicy {
  276|      2|        self.encode_policy
  277|      2|    }
  278|       |
  279|       |    /// Returns the configured malformed-input action.
  280|       |    ///
  281|       |    /// # Returns
  282|       |    ///
  283|       |    /// Returns the action used when source input is malformed.
  284|       |    #[inline]
  285|       |    #[must_use]
  286|      2|    pub const fn malformed_action(&self) -> MalformedAction {
  287|      2|        self.decode_policy.malformed_action()
  288|      2|    }
  289|       |
  290|       |    /// Returns the configured source replacement character.
  291|       |    ///
  292|       |    /// # Returns
  293|       |    ///
  294|       |    /// Returns the character emitted when malformed source input is replaced.
  295|       |    #[inline]
  296|       |    #[must_use]
  297|      2|    pub const fn decode_replacement(&self) -> char {
  298|      2|        self.decode_policy.replacement()
  299|      2|    }
  300|       |
  301|       |    /// Returns the configured unmappable-character action.
  302|       |    ///
  303|       |    /// # Returns
  304|       |    ///
  305|       |    /// Returns the action used when the target charset cannot represent a
  306|       |    /// character.
  307|       |    #[inline]
  308|       |    #[must_use]
  309|      4|    pub const fn unmappable_action(&self) -> UnmappableAction {
  310|      4|        self.encode_policy.unmappable_action()
  311|      4|    }
  312|       |
  313|       |    /// Returns the configured target replacement character.
  314|       |    ///
  315|       |    /// # Returns
  316|       |    ///
  317|       |    /// Returns the character encoded when unmappable target input is replaced.
  318|       |    #[inline]
  319|       |    #[must_use]
  320|      3|    pub const fn replacement(&self) -> char {
  321|      3|        self.encode_policy.replacement()
  322|      3|    }
  323|       |
  324|       |    /// Maps a low-level converter error into the charset-specific error model.
  325|       |    ///
  326|       |    /// # Parameters
  327|       |    ///
  328|       |    /// - `error`: Error returned by this converter's transcode operations.
  329|       |    ///
  330|       |    /// # Returns
  331|       |    ///
  332|       |    /// Returns a [`CharsetConvertError`] carrying this converter's source or
  333|       |    /// target charset as appropriate.
  334|       |    #[must_use]
  335|       |    #[inline]
  336|      1|    pub fn map_transcode_error(&self, error: TranscodeConvertErrorOf<D, E>) -> CharsetConvertError {
  337|      1|        CharsetConvertError::from_transcode_error::<D, E>(self.source_charset(), self.target_charset(), error)
  338|      1|    }
  339|       |
  340|       |    /// Resets converter state.
  341|       |    ///
  342|       |    /// # Errors
  343|       |    ///
  344|       |    /// Returns a transcode convert error when reset output cannot be written or
  345|       |    /// when source decode reset or target encode reset reports a
  346|       |    /// charset-domain error.
  347|       |    #[inline]
  348|      4|    pub fn reset(
  349|      4|        &mut self,
  350|      4|        output: &mut [E::Unit],
  351|      4|        output_index: usize,
  352|      4|    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
  353|      4|        self.engine.reset(output, output_index)
  354|      4|    }
  355|       |
  356|       |    /// Converts source units into target units.
  357|       |    ///
  358|       |    /// # Errors
  359|       |    ///
  360|       |    /// Returns a transcode convert error when source decoding or target
  361|       |    /// encoding fails, or when framework-level index and capacity checks
  362|       |    /// fail.
  363|       |    #[inline]
  364|     93|    pub fn transcode(
  365|     93|        &mut self,
  366|     93|        input: &[D::Unit],
  367|     93|        input_index: usize,
  368|     93|        output: &mut [E::Unit],
  369|     93|        output_index: usize,
  370|     93|    ) -> Result<TranscodeProgress, TranscodeConvertErrorOf<D, E>> {
  371|     93|        self.engine.transcode(input, input_index, output, output_index)
  372|     93|    }
  373|       |
  374|       |    /// Converts source units after the caller has established end of input.
  375|       |    ///
  376|       |    /// # Errors
  377|       |    ///
  378|       |    /// Returns a transcode convert error when EOF source input violates the
  379|       |    /// configured decode policy, target encoding fails, or framework-level
  380|       |    /// index and capacity checks fail.
  381|       |    #[inline]
  382|      4|    pub fn transcode_eof(
  383|      4|        &mut self,
  384|      4|        input: &[D::Unit],
  385|      4|        input_index: usize,
  386|      4|        output: &mut [E::Unit],
  387|      4|        output_index: usize,
  388|      4|    ) -> Result<TranscodeProgress, TranscodeConvertErrorOf<D, E>> {
  389|      4|        self.engine.transcode_eof(input, input_index, output, output_index)
  390|      4|    }
  391|       |
  392|       |    /// Finishes retained converter output.
  393|       |    ///
  394|       |    /// # Errors
  395|       |    ///
  396|       |    /// Returns a transcode convert error when pending or final decoded values
  397|       |    /// cannot be encoded or written to the target output buffer.
  398|       |    #[inline]
  399|     74|    pub fn finish(
  400|     74|        &mut self,
  401|     74|        output: &mut [E::Unit],
  402|     74|        output_index: usize,
  403|     74|    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
  404|     74|        self.engine.finish(output, output_index)
  405|     74|    }
  406|       |
  407|       |    /// Runs a complete `reset -> transcode -> finish` conversion stream.
  408|       |    ///
  409|       |    /// # Errors
  410|       |    ///
  411|       |    /// Returns a transcode convert error when source decoding or target
  412|       |    /// encoding fails under the configured policies, or when the supplied
  413|       |    /// output buffer is too small.
  414|       |    #[inline]
  415|      5|    pub fn transcode_complete_into(
  416|      5|        &mut self,
  417|      5|        input: &[D::Unit],
  418|      5|        output: &mut [E::Unit],
  419|      5|    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
  420|      5|        <Self as Transcoder>::transcode_complete_into(self, input, output)
  421|      5|    }
  422|       |
  423|       |    /// Returns the default encode policy that can be represented by `target`.
  424|       |    ///
  425|       |    /// # Panics
  426|       |    ///
  427|       |    /// Panics when neither the default replacement nor the fallback replacement
  428|       |    /// can be encoded by `target`. This panic is intentional: reaching this
  429|       |    /// branch means the target codec implementation violates the replacement
  430|       |    /// fallback invariant and should fail fast.
  431|     23|    fn default_encode_policy(target: &E) -> (CharsetEncodePolicy, CharsetEncodeHooks) {
  432|     23|        let policy = CharsetEncodePolicy::default_for(target).unwrap_or_else(|error| {
                                                                                                   ^1
  433|       |            // This panic is intentional. If default replacement selection gets
  434|       |            // here, the target codec cannot encode even the required fallback
  435|       |            // `?`. That violates the codec invariant expected by this API, so
  436|       |            // construction fails fast to expose the broken codec implementation.
  437|      1|            panic!(
  438|       |                "cannot initialize CharsetConverter target for {:?}: neither {:?} nor {:?} is encodable ({error})",
  439|      1|                target.charset(),
  440|       |                CharsetEncodePolicy::DEFAULT_REPLACEMENT,
  441|       |                CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
  442|       |            )
  443|       |        });
  444|     23|        let hooks = CharsetEncoder::<E>::create_hooks(target, policy)
  445|       |            // A policy chosen by `default_encode_policy` must be encodable;
  446|       |            // failing here can only mean the target codec violates the
  447|       |            // replacement fallback invariant. This panic is deliberate for the
  448|       |            // same fail-fast reason as the default-policy panic above.
  449|     23|            .expect(
  450|     23|                "validated default encode policy should create hooks; \
  451|     23|                 failure means the codec violated its replacement invariant",
  452|       |            );
  453|     23|        (policy, hooks)
  454|     23|    }
  455|       |}
  456|       |
  457|       |impl<D, E> Transcoder for CharsetConverter<D, E>
  458|       |where
  459|       |    D: CharsetCodec,
  460|       |    E: CharsetCodec,
  461|       |{
  462|       |    type Input = D::Unit;
  463|       |    type Output = E::Unit;
  464|       |    type Error = TranscodeConvertErrorOf<D, E>;
  465|       |
  466|       |    /// Returns the target-side upper bound for converted output units.
  467|       |    #[inline]
  468|      9|    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
  469|      9|        self.engine.max_transcode_output_len(input_len)
  470|      9|    }
  471|       |
  472|       |    /// Returns the maximum target units needed to finalize pending conversion
  473|       |    /// state.
  474|       |    #[inline]
  475|     15|    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
  476|     15|        self.engine.max_finish_output_len()
  477|     15|    }
  478|       |
  479|       |    /// Returns the maximum target units emitted when resetting stream state.
  480|       |    #[inline]
  481|      8|    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
  482|      8|        self.engine.max_reset_output_len()
  483|      8|    }
  484|       |
  485|       |    /// Clears any pending decoded character and emits stream-start encode
  486|       |    /// output.
  487|       |    #[inline]
  488|     97|    fn reset(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<usize, Self::Error> {
  489|     97|        self.engine.reset(output, output_index)
  490|     97|    }
  491|       |
  492|       |    /// Converts source units to target units through the configured decoder and
  493|       |    /// encoder.
  494|       |    ///
  495|       |    /// # Errors
  496|       |    ///
  497|       |    /// Returns framework failures when indices or output capacity are invalid,
  498|       |    /// and domain errors when source decoding or target encoding fails.
  499|       |    #[inline]
  500|      1|    fn transcode(
  501|      1|        &mut self,
  502|      1|        input: &[D::Unit],
  503|      1|        input_index: usize,
  504|      1|        output: &mut [E::Unit],
  505|      1|        output_index: usize,
  506|      1|    ) -> Result<TranscodeProgress, Self::Error> {
  507|      1|        self.engine.transcode(input, input_index, output, output_index)
  508|      1|    }
  509|       |
  510|       |    /// Converts source units after the caller has established end of input.
  511|       |    ///
  512|       |    /// # Errors
  513|       |    ///
  514|       |    /// Returns framework failures when indices or output capacity are invalid,
  515|       |    /// and domain errors when EOF source input violates the configured decode
  516|       |    /// policy or target encoding policy.
  517|       |    #[inline]
  518|      4|    fn transcode_eof(
  519|      4|        &mut self,
  520|      4|        input: &[D::Unit],
  521|      4|        input_index: usize,
  522|      4|        output: &mut [E::Unit],
  523|      4|        output_index: usize,
  524|      4|    ) -> Result<TranscodeProgress, Self::Error> {
  525|      4|        CharsetConverter::transcode_eof(self, input, input_index, output, output_index)
  526|      4|    }
  527|       |
  528|       |    /// Finalizes internally retained decoded characters and policy hook state.
  529|       |    ///
  530|       |    /// # Parameters
  531|       |    ///
  532|       |    /// - `output`: Complete output slice visible to the converter.
  533|       |    /// - `output_index`: Absolute output index where writing starts.
  534|       |    ///
  535|       |    /// # Returns
  536|       |    ///
  537|       |    /// Returns the number of target units written during finalization.
  538|       |    ///
  539|       |    /// # Errors
  540|       |    ///
  541|       |    /// Returns framework failures when `output_index` is invalid or output
  542|       |    /// capacity is insufficient, and domain errors when encoding pending or
  543|       |    /// final decoded characters violates target charset policy.
  544|       |    #[inline]
  545|      4|    fn finish(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<usize, Self::Error> {
  546|      4|        self.engine.finish(output, output_index)
  547|      4|    }
  548|       |}
  549|       |
  550|       |impl<D, E> TranscodeConverter for CharsetConverter<D, E>
  551|       |where
  552|       |    D: CharsetCodec,
  553|       |    E: CharsetCodec,
  554|       |{
  555|       |    type DecodeError = CharsetDecodeError;
  556|       |    type EncodeError = CharsetEncodeError;
  557|       |    type Value = char;
  558|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/decode/charset_decode_hooks.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::CapacityError;
    9|       |use qubit_codec::TranscodeDecodeError;
   10|       |use qubit_codec::TranscodeDecodeErrorOf;
   11|       |use qubit_codec::engine::DecodeContext;
   12|       |use qubit_codec::engine::DecodeIncompleteAction;
   13|       |use qubit_codec::engine::DecodeInvalidAction;
   14|       |use qubit_codec::engine::TranscodeDecodeHooks;
   15|       |
   16|       |use crate::CharsetCodec;
   17|       |use crate::CharsetDecodeError;
   18|       |use crate::MalformedAction;
   19|       |
   20|       |/// Malformed-input policy hooks used by [`super::CharsetDecoder`].
   21|       |#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   22|       |pub(crate) struct CharsetDecodeHooks {
   23|       |    /// Action used for malformed input units.
   24|       |    pub(super) malformed_action: MalformedAction,
   25|       |    /// Replacement character used by [`MalformedAction::Replace`].
   26|       |    pub(super) replacement: char,
   27|       |}
   28|       |
   29|       |impl CharsetDecodeHooks {
   30|       |    /// Creates charset decode hooks.
   31|       |    ///
   32|       |    /// # Parameters
   33|       |    ///
   34|       |    /// - `malformed_action`: Initial malformed-input policy.
   35|       |    /// - `replacement`: Replacement character used by replace policy.
   36|       |    ///
   37|       |    /// # Returns
   38|       |    ///
   39|       |    /// Returns hooks carrying the supplied policy.
   40|       |    #[must_use]
   41|       |    #[inline]
   42|    943|    pub(crate) const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
   43|    943|        Self {
   44|    943|            malformed_action,
   45|    943|            replacement,
   46|    943|        }
   47|    943|    }
   48|       |}
   49|       |
   50|       |impl<C> TranscodeDecodeHooks<C> for CharsetDecodeHooks
   51|       |where
   52|       |    C: CharsetCodec,
   53|       |{
   54|       |    /// Returns the maximum number of characters decoded from `input_len` units.
   55|       |    #[inline]
   56|  1.55k|    fn max_transcode_output_len(&self, _codec: &C, input_len: usize) -> Result<usize, CapacityError> {
   57|  1.55k|        Ok(input_len)
   58|  1.55k|    }
   59|       |
   60|       |    /// Handles a charset decode failure during `transcode`.
   61|  26.1k|    fn handle_invalid_decode(
   62|  26.1k|        &mut self,
   63|  26.1k|        _codec: &mut C,
   64|  26.1k|        error: &CharsetDecodeError,
   65|  26.1k|        _consumed: Option<core::num::NonZeroUsize>,
   66|  26.1k|        context: DecodeContext,
   67|  26.1k|    ) -> Result<DecodeInvalidAction<char>, TranscodeDecodeErrorOf<C>> {
   68|  26.1k|        if error.kind().is_malformed_input() {
   69|  26.1k|            let consumed = error.consumed().expect("malformed decode errors carry consumed width");
   70|  26.1k|            return match self.malformed_action {
   71|     21|                MalformedAction::Report => Err(TranscodeDecodeError::domain_main(*error, context.input_index())),
   72|     20|                MalformedAction::Ignore => Ok(DecodeInvalidAction::Skip { consumed }),
   73|  26.1k|                MalformedAction::Replace => Ok(DecodeInvalidAction::Emit {
   74|  26.1k|                    value: self.replacement,
   75|  26.1k|                    consumed,
   76|  26.1k|                }),
   77|       |            };
   78|      1|        }
   79|      1|        Err(TranscodeDecodeError::domain_main(*error, context.input_index()))
   80|  26.1k|    }
   81|       |
   82|       |    /// Selects the malformed-input action for a source tail left incomplete at
   83|       |    /// end of input.
   84|       |    ///
   85|       |    /// Incomplete EOF tails use the same policy as malformed source input.
   86|       |    /// The engine preserves the optional codec error when reporting is chosen.
   87|     75|    fn handle_incomplete_decode(
   88|     75|        &mut self,
   89|     75|        _codec: &mut C,
   90|     75|        _source: Option<&CharsetDecodeError>,
   91|     75|        _required_total: core::num::NonZeroUsize,
   92|     75|        _context: DecodeContext,
   93|     75|    ) -> Result<DecodeIncompleteAction<char>, TranscodeDecodeErrorOf<C>> {
   94|     75|        match self.malformed_action {
   95|      2|            MalformedAction::Report => Ok(DecodeIncompleteAction::Reject),
   96|      2|            MalformedAction::Ignore => Ok(DecodeIncompleteAction::Skip),
   97|     71|            MalformedAction::Replace => Ok(DecodeIncompleteAction::Emit {
   98|     71|                value: self.replacement,
   99|     71|            }),
  100|       |        }
  101|     75|    }
  102|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/decode/charset_decode_policy.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use crate::MalformedAction;
    9|       |
   10|       |/// Malformed-input and incomplete-EOF policy used by charset decoders and
   11|       |/// converters.
   12|       |#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   13|       |pub struct CharsetDecodePolicy {
   14|       |    /// Action used for malformed input units and incomplete tails at EOF.
   15|       |    malformed_action: MalformedAction,
   16|       |    /// Replacement character used by [`MalformedAction::Replace`].
   17|       |    replacement: char,
   18|       |}
   19|       |
   20|       |impl CharsetDecodePolicy {
   21|       |    /// Default replacement character used when malformed input is replaced.
   22|       |    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';
   23|       |
   24|       |    /// Creates a policy for malformed input and incomplete tails at EOF.
   25|       |    #[must_use]
   26|       |    #[inline]
   27|    959|    pub const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
   28|    959|        Self {
   29|    959|            malformed_action,
   30|    959|            replacement,
   31|    959|        }
   32|    959|    }
   33|       |
   34|       |    /// Creates a replacement policy.
   35|       |    #[must_use]
   36|       |    #[inline]
   37|    905|    pub const fn replace(replacement: char) -> Self {
   38|    905|        Self::new(MalformedAction::Replace, replacement)
   39|    905|    }
   40|       |
   41|       |    /// Creates an ignore policy with the default replacement retained for
   42|       |    /// metadata.
   43|       |    #[must_use]
   44|       |    #[inline]
   45|     24|    pub const fn ignore() -> Self {
   46|     24|        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
   47|     24|    }
   48|       |
   49|       |    /// Creates an ignore policy with explicit replacement metadata.
   50|       |    #[must_use]
   51|       |    #[inline]
   52|     26|    pub const fn ignore_with_replacement(replacement: char) -> Self {
   53|     26|        Self::new(MalformedAction::Ignore, replacement)
   54|     26|    }
   55|       |
   56|       |    /// Creates a report policy with the default replacement retained for
   57|       |    /// metadata.
   58|       |    #[must_use]
   59|       |    #[inline]
   60|     28|    pub const fn report() -> Self {
   61|     28|        Self::new(MalformedAction::Report, Self::DEFAULT_REPLACEMENT)
   62|     28|    }
   63|       |
   64|       |    /// Returns the malformed-input action.
   65|       |    #[must_use]
   66|       |    #[inline]
   67|    954|    pub const fn malformed_action(self) -> MalformedAction {
   68|    954|        self.malformed_action
   69|    954|    }
   70|       |
   71|       |    /// Returns the replacement character.
   72|       |    #[must_use]
   73|       |    #[inline]
   74|    953|    pub const fn replacement(self) -> char {
   75|    953|        self.replacement
   76|    953|    }
   77|       |}
   78|       |
   79|       |impl Default for CharsetDecodePolicy {
   80|       |    #[inline]
   81|    879|    fn default() -> Self {
   82|    879|        Self::replace(Self::DEFAULT_REPLACEMENT)
   83|    879|    }
   84|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/decode/charset_decoder.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::CapacityError;
    9|       |use qubit_codec::Codec;
   10|       |use qubit_codec::TranscodeDecodeErrorOf;
   11|       |use qubit_codec::TranscodeDecoder;
   12|       |use qubit_codec::TranscodeProgress;
   13|       |use qubit_codec::Transcoder;
   14|       |use qubit_codec::engine::TranscodeDecodeEngine;
   15|       |
   16|       |use super::charset_decode_hooks::CharsetDecodeHooks;
   17|       |use super::charset_decode_policy::CharsetDecodePolicy;
   18|       |use crate::BomDetectStatus;
   19|       |use crate::CharsetCodec;
   20|       |use crate::CharsetDecodeError;
   21|       |use crate::MalformedAction;
   22|       |use crate::UnicodeBom;
   23|       |
   24|       |/// Converts units of one charset into Unicode scalar values.
   25|       |///
   26|       |/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
   27|       |/// configured [`MalformedAction`] whenever the codec reports malformed input
   28|       |/// or an incomplete tail is encountered at EOF.
   29|       |/// The decoder asks the wrapped codec whether one value can be decoded from the
   30|       |/// currently available units. If the codec reports a valid incomplete prefix,
   31|       |/// the tail is left in the caller-provided input slice and
   32|       |/// [`qubit_codec::TranscodeStatus::NeedInput`] is returned. Callers must handle
   33|       |/// incomplete EOF tails through [`Transcoder::transcode_eof`] before calling
   34|       |/// [`Transcoder::finish`].
   35|       |///
   36|       |/// # Type Parameters
   37|       |///
   38|       |/// - `C`: Low-level charset codec used to decode source storage units into one
   39|       |///   Unicode scalar value.
   40|       |#[derive(Debug)]
   41|       |pub struct CharsetDecoder<C>
   42|       |where
   43|       |    C: CharsetCodec,
   44|       |{
   45|       |    /// Common buffered decode engine.
   46|       |    engine: TranscodeDecodeEngine<C, CharsetDecodeHooks>,
   47|       |    /// Public malformed-input policy metadata.
   48|       |    policy: CharsetDecodePolicy,
   49|       |}
   50|       |
   51|       |impl<C> CharsetDecoder<C>
   52|       |where
   53|       |    C: CharsetCodec,
   54|       |{
   55|       |    /// Creates a decoder with default replacement policy.
   56|       |    ///
   57|       |    /// # Parameters
   58|       |    ///
   59|       |    /// - `codec`: Low-level charset codec used to decode input units.
   60|       |    ///
   61|       |    /// # Returns
   62|       |    ///
   63|       |    /// Returns a decoder whose malformed action is [`MalformedAction::Replace`]
   64|       |    /// and whose replacement character is `U+FFFD`.
   65|       |    #[inline(always)]
   66|       |    #[must_use]
   67|    796|    pub fn new(codec: C) -> Self {
   68|    796|        Self::with_policy(codec, CharsetDecodePolicy::default())
   69|    796|    }
   70|       |
   71|       |    /// Creates a decoder with an explicit malformed-input policy.
   72|       |    ///
   73|       |    /// # Parameters
   74|       |    ///
   75|       |    /// - `codec`: Low-level charset codec used to decode input units.
   76|       |    /// - `policy`: Malformed-input policy used by the decoder.
   77|       |    ///
   78|       |    /// # Returns
   79|       |    ///
   80|       |    /// Returns a decoder configured with `policy`.
   81|       |    #[must_use]
   82|    839|    pub fn with_policy(codec: C, policy: CharsetDecodePolicy) -> Self {
   83|    839|        let hooks = CharsetDecodeHooks::new(policy.malformed_action(), policy.replacement());
   84|    839|        Self {
   85|    839|            engine: TranscodeDecodeEngine::new(codec, hooks),
   86|    839|            policy,
   87|    839|        }
   88|    839|    }
   89|       |
   90|       |    /// Returns the charset decoded by the wrapped codec.
   91|       |    ///
   92|       |    /// # Returns
   93|       |    ///
   94|       |    /// Returns the charset reported by the low-level codec.
   95|       |    #[inline(always)]
   96|       |    #[must_use]
   97|    514|    pub fn charset(&self) -> crate::Charset {
   98|    514|        self.codec().charset()
   99|    514|    }
  100|       |
  101|       |    /// Returns the wrapped low-level codec.
  102|       |    ///
  103|       |    /// # Returns
  104|       |    ///
  105|       |    /// Returns a shared reference to the codec owned by this decoder.
  106|       |    #[inline(always)]
  107|       |    #[must_use]
  108|    515|    pub fn codec(&self) -> &C {
  109|    515|        self.engine.codec()
  110|    515|    }
  111|       |
  112|       |    /// Returns the wrapped low-level codec mutably.
  113|       |    ///
  114|       |    /// Mutating codec configuration while a stream is active can invalidate
  115|       |    /// buffered decode state. Reset the decoder before continuing with the
  116|       |    /// modified codec.
  117|       |    ///
  118|       |    /// # Returns
  119|       |    ///
  120|       |    /// Returns a mutable reference to the codec owned by this decoder.
  121|       |    #[inline(always)]
  122|       |    #[must_use]
  123|      1|    pub fn codec_mut(&mut self) -> &mut C {
  124|      1|        self.engine.codec_mut()
  125|      1|    }
  126|       |
  127|       |    /// Consumes the decoder and returns its codec.
  128|       |    ///
  129|       |    /// Decoder policy and hook state are discarded.
  130|       |    ///
  131|       |    /// # Returns
  132|       |    ///
  133|       |    /// Returns the low-level codec owned by this decoder.
  134|       |    #[must_use]
  135|       |    #[inline]
  136|      1|    pub fn into_codec(self) -> C {
  137|      1|        let (codec, _) = self.engine.into_parts();
  138|      1|        codec
  139|      1|    }
  140|       |
  141|       |    /// Returns the configured malformed-input action.
  142|       |    ///
  143|       |    /// # Returns
  144|       |    ///
  145|       |    /// Returns the action used when source input is malformed.
  146|       |    #[inline(always)]
  147|       |    #[must_use]
  148|      2|    pub const fn malformed_action(&self) -> MalformedAction {
  149|      2|        self.policy.malformed_action()
  150|      2|    }
  151|       |
  152|       |    /// Returns the configured replacement character.
  153|       |    ///
  154|       |    /// # Returns
  155|       |    ///
  156|       |    /// Returns the character emitted when [`MalformedAction::Replace`] is used.
  157|       |    #[inline(always)]
  158|       |    #[must_use]
  159|      3|    pub const fn replacement(&self) -> char {
  160|      3|        self.policy.replacement()
  161|      3|    }
  162|       |
  163|       |    /// Runs decoder reset.
  164|       |    ///
  165|       |    /// # Errors
  166|       |    ///
  167|       |    /// Returns a transcode decode error when `output_index` is invalid, output
  168|       |    /// capacity is insufficient, or decoder reset emits a charset-domain
  169|       |    /// error.
  170|       |    #[inline]
  171|      3|    pub fn reset(&mut self, output: &mut [char], output_index: usize) -> Result<usize, TranscodeDecodeErrorOf<C>> {
  172|      3|        self.engine.reset(output, output_index)
  173|      3|    }
  174|       |
  175|       |    /// Decodes source units into Unicode scalar values.
  176|       |    ///
  177|       |    /// # Errors
  178|       |    ///
  179|       |    /// Returns a transcode decode error when indices are invalid, output
  180|       |    /// capacity is insufficient, input is malformed under the configured
  181|       |    /// policy, or the codec reports another decode-domain error.
  182|       |    #[inline]
  183|     61|    pub fn transcode(
  184|     61|        &mut self,
  185|     61|        input: &[C::Unit],
  186|     61|        input_index: usize,
  187|     61|        output: &mut [char],
  188|     61|        output_index: usize,
  189|     61|    ) -> Result<TranscodeProgress, TranscodeDecodeErrorOf<C>> {
  190|     61|        self.engine.transcode(input, input_index, output, output_index)
  191|     61|    }
  192|       |
  193|       |    /// Decodes source units after the caller has established end of input.
  194|       |    #[inline]
  195|      1|    pub fn transcode_eof(
  196|      1|        &mut self,
  197|      1|        input: &[C::Unit],
  198|      1|        input_index: usize,
  199|      1|        output: &mut [char],
  200|      1|        output_index: usize,
  201|      1|    ) -> Result<TranscodeProgress, TranscodeDecodeErrorOf<C>> {
  202|      1|        self.engine.transcode_eof(input, input_index, output, output_index)
  203|      1|    }
  204|       |
  205|       |    /// Maps a decoder transcode error into a charset decode error.
  206|       |    ///
  207|       |    /// Framework failures are attached to this decoder's charset. Codec-domain
  208|       |    /// errors retain their original charset error.
  209|       |    #[must_use]
  210|       |    #[inline]
  211|      1|    pub fn map_transcode_error(&self, error: TranscodeDecodeErrorOf<C>) -> CharsetDecodeError {
  212|      1|        CharsetDecodeError::from_transcode_error(self.charset(), error)
  213|      1|    }
  214|       |
  215|       |    /// Finishes decoder-owned final output after EOF.
  216|       |    ///
  217|       |    /// # Errors
  218|       |    ///
  219|       |    /// Returns a transcode decode error when finalization output cannot be
  220|       |    /// written or when the codec reports a final decode-domain error.
  221|       |    #[inline]
  222|     18|    pub fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<usize, TranscodeDecodeErrorOf<C>> {
  223|     18|        self.engine.finish(output, output_index)
  224|     18|    }
  225|       |
  226|       |    /// Runs a complete `reset -> transcode -> finish` decode stream.
  227|       |    ///
  228|       |    /// # Errors
  229|       |    ///
  230|       |    /// Returns a transcode decode error when the supplied output buffer is too
  231|       |    /// small, the configured policy reports an incomplete sequence, or the
  232|       |    /// codec reports another charset-domain decode error.
  233|       |    #[inline]
  234|    774|    pub fn transcode_complete_into(
  235|    774|        &mut self,
  236|    774|        input: &[C::Unit],
  237|    774|        output: &mut [char],
  238|    774|    ) -> Result<usize, TranscodeDecodeErrorOf<C>> {
  239|    774|        <Self as Transcoder>::transcode_complete_into(self, input, output)
  240|    774|    }
  241|       |}
  242|       |
  243|       |impl<C> CharsetDecoder<C>
  244|       |where
  245|       |    C: CharsetCodec + Codec<Unit = u8>,
  246|       |{
  247|       |    /// Detects and strips a Unicode byte order mark from a closed byte input.
  248|       |    ///
  249|       |    /// This helper treats `input` as EOF-reached input. Streaming callers that
  250|       |    /// still may receive more prefix bytes should use
  251|       |    /// [`Self::detect_and_strip_bom_progress`] to avoid stripping ambiguous BOM
  252|       |    /// prefixes too early.
  253|       |    ///
  254|       |    /// # Returns
  255|       |    ///
  256|       |    /// Returns the detected BOM, if any, plus the input slice after the BOM
  257|       |    /// prefix. If no BOM is detected, returns `None` and the original input
  258|       |    /// slice.
  259|       |    #[must_use]
  260|      3|    pub fn detect_and_strip_bom(input: &[u8]) -> (Option<UnicodeBom>, &[u8]) {
  261|      3|        match Self::detect_and_strip_bom_progress(input, true) {
  262|      2|            (BomDetectStatus::Match(bom), stripped) => (Some(bom), stripped),
  263|      1|            (BomDetectStatus::Pending | BomDetectStatus::None, stripped) => (None, stripped),
  264|       |        }
  265|      3|    }
  266|       |
  267|       |    /// Detects and strips a Unicode byte order mark with an explicit EOF
  268|       |    /// signal.
  269|       |    ///
  270|       |    /// # Parameters
  271|       |    ///
  272|       |    /// - `input`: Bytes currently available from the beginning of the stream.
  273|       |    /// - `eof`: Whether no more bytes can arrive.
  274|       |    ///
  275|       |    /// # Returns
  276|       |    ///
  277|       |    /// Returns the incremental BOM detection status plus the original input
  278|       |    /// slice for [`BomDetectStatus::Pending`] and [`BomDetectStatus::None`],
  279|       |    /// or the input slice after the BOM prefix for [`BomDetectStatus::Match`].
  280|       |    #[must_use]
  281|      7|    pub fn detect_and_strip_bom_progress(input: &[u8], eof: bool) -> (BomDetectStatus, &[u8]) {
  282|      7|        match UnicodeBom::detect_progress(input, eof) {
  283|      4|            BomDetectStatus::Match(bom) => (BomDetectStatus::Match(bom), &input[bom.bytes().len()..]),
  284|      3|            status @ (BomDetectStatus::Pending | BomDetectStatus::None) => (status, input),
  285|       |        }
  286|      7|    }
  287|       |}
  288|       |
  289|       |impl<C> Transcoder for CharsetDecoder<C>
  290|       |where
  291|       |    C: CharsetCodec,
  292|       |{
  293|       |    type Input = C::Unit;
  294|       |    type Output = char;
  295|       |    type Error = TranscodeDecodeErrorOf<C>;
  296|       |
  297|       |    /// Returns the maximum number of characters decoded from `input_len` units.
  298|       |    #[inline(always)]
  299|  1.54k|    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
  300|  1.54k|        self.engine.max_transcode_output_len(input_len)
  301|  1.54k|    }
  302|       |
  303|       |    /// Returns the maximum number of characters emitted by finishing internal
  304|       |    /// state.
  305|       |    #[inline(always)]
  306|  1.55k|    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
  307|  1.55k|        self.engine.max_finish_output_len()
  308|  1.55k|    }
  309|       |
  310|       |    /// Returns the maximum characters emitted when resetting stream state.
  311|       |    #[inline(always)]
  312|  1.54k|    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
  313|  1.54k|        self.engine.max_reset_output_len()
  314|  1.54k|    }
  315|       |
  316|       |    /// Runs decoder reset while keeping decoder policy.
  317|       |    #[inline(always)]
  318|    833|    fn reset(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
  319|    833|        self.engine.reset(output, output_index)
  320|    833|    }
  321|       |
  322|       |    /// Decodes source units into Unicode scalar values while applying malformed
  323|       |    /// policy.
  324|       |    #[inline(always)]
  325|      1|    fn transcode(
  326|      1|        &mut self,
  327|      1|        input: &[C::Unit],
  328|      1|        input_index: usize,
  329|      1|        output: &mut [char],
  330|      1|        output_index: usize,
  331|      1|    ) -> Result<TranscodeProgress, Self::Error> {
  332|      1|        self.engine.transcode(input, input_index, output, output_index)
  333|      1|    }
  334|       |
  335|       |    #[inline(always)]
  336|    774|    fn transcode_eof(
  337|    774|        &mut self,
  338|    774|        input: &[C::Unit],
  339|    774|        input_index: usize,
  340|    774|        output: &mut [char],
  341|    774|        output_index: usize,
  342|    774|    ) -> Result<TranscodeProgress, Self::Error> {
  343|    774|        self.engine.transcode_eof(input, input_index, output, output_index)
  344|    774|    }
  345|       |
  346|       |    /// Finishes decoder-owned final output after EOF.
  347|       |    #[inline(always)]
  348|    773|    fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
  349|    773|        self.engine.finish(output, output_index)
  350|    773|    }
  351|       |}
  352|       |
  353|       |impl<C> TranscodeDecoder for CharsetDecoder<C>
  354|       |where
  355|       |    C: CharsetCodec,
  356|       |{
  357|       |    type DecodeError = CharsetDecodeError;
  358|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/encode/charset_encode_hooks.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::TranscodeEncodeError;
    9|       |use qubit_codec::TranscodeEncodeErrorOf;
   10|       |use qubit_codec::engine::EncodeContext;
   11|       |use qubit_codec::engine::EncodeUnencodableAction;
   12|       |use qubit_codec::engine::TranscodeEncodeHooks;
   13|       |
   14|       |use crate::CharsetCodec;
   15|       |use crate::CharsetEncodeError;
   16|       |use crate::CharsetEncodeErrorKind;
   17|       |use crate::CharsetEncodeResult;
   18|       |use crate::UnmappableAction;
   19|       |
   20|       |/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
   21|       |#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   22|       |pub(crate) struct CharsetEncodeHooks {
   23|       |    /// Action used for unmappable input characters.
   24|       |    pub(super) unmappable_action: UnmappableAction,
   25|       |    /// Replacement character used by [`UnmappableAction::Replace`].
   26|       |    pub(super) replacement: char,
   27|       |}
   28|       |
   29|       |impl CharsetEncodeHooks {
   30|       |    /// Creates charset encode hooks.
   31|       |    ///
   32|       |    /// # Parameters
   33|       |    ///
   34|       |    /// - `unmappable_action`: Initial unmappable-character policy.
   35|       |    /// - `replacement`: Initial replacement character.
   36|       |    ///
   37|       |    /// # Returns
   38|       |    ///
   39|       |    /// Returns hooks configured with the supplied policy.
   40|       |    #[must_use]
   41|       |    #[inline]
   42|    671|    pub(crate) const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
   43|    671|        Self {
   44|    671|            unmappable_action,
   45|    671|            replacement,
   46|    671|        }
   47|    671|    }
   48|       |}
   49|       |
   50|       |impl<C> TranscodeEncodeHooks<C> for CharsetEncodeHooks
   51|       |where
   52|       |    C: CharsetCodec,
   53|       |{
   54|       |    /// Handles one character rejected by the charset codec.
   55|       |    #[inline]
   56|     17|    fn handle_unencodable_encode(
   57|     17|        &mut self,
   58|     17|        codec: &mut C,
   59|     17|        context: &EncodeContext<'_, char>,
   60|     17|    ) -> Result<EncodeUnencodableAction<char>, TranscodeEncodeErrorOf<C>> {
   61|     17|        let ch = *context.input_value();
   62|     17|        let input_index = context.input_index();
   63|     17|        let error = unmappable_error(codec, ch, input_index);
   64|     17|        match self.unmappable_action {
   65|      5|            UnmappableAction::Report => Err(TranscodeEncodeError::domain_main(error, input_index)),
   66|      3|            UnmappableAction::Ignore => Ok(EncodeUnencodableAction::Skip),
   67|      9|            UnmappableAction::Replace => Ok(EncodeUnencodableAction::replace(self.replacement)),
   68|       |        }
   69|     17|    }
   70|       |}
   71|       |
   72|       |/// Returns the encoded width of a replacement character.
   73|    646|pub(super) fn replacement_len<C>(codec: &C, ch: char) -> CharsetEncodeResult<usize>
   74|    646|where
   75|    646|    C: CharsetCodec,
   76|       |{
   77|    646|    if !codec.can_encode_value(&ch) {
   78|      3|        return Err(unmappable_error(codec, ch, 0));
   79|    643|    }
   80|    643|    Ok(codec.encode_len(&ch))
   81|    646|}
   82|       |
   83|       |/// Creates an unmappable-character error for `ch`.
   84|     20|fn unmappable_error<C>(codec: &C, ch: char, index: usize) -> CharsetEncodeError
   85|     20|where
   86|     20|    C: CharsetCodec,
   87|       |{
   88|     20|    let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
   89|     20|    CharsetEncodeError::new(codec.charset(), kind, index)
   90|     20|}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/encode/charset_encode_policy.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use crate::CharsetCodec;
    9|       |use crate::CharsetEncodeError;
   10|       |use crate::CharsetEncodeErrorKind;
   11|       |use crate::UnmappableAction;
   12|       |
   13|       |/// Unmappable-input policy used by charset encoders and converters.
   14|       |#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   15|       |pub struct CharsetEncodePolicy {
   16|       |    /// Action used for unmappable input characters.
   17|       |    unmappable_action: UnmappableAction,
   18|       |    /// Replacement character used by [`UnmappableAction::Replace`].
   19|       |    replacement: char,
   20|       |}
   21|       |
   22|       |impl CharsetEncodePolicy {
   23|       |    /// Default replacement character used when unmappable input is replaced.
   24|       |    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';
   25|       |    /// Fallback replacement used when the default replacement is unmappable.
   26|       |    pub const DEFAULT_FALLBACK_REPLACEMENT: char = '?';
   27|       |
   28|       |    /// Creates an unmappable-input policy.
   29|       |    #[must_use]
   30|       |    #[inline]
   31|    690|    pub const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
   32|    690|        Self {
   33|    690|            unmappable_action,
   34|    690|            replacement,
   35|    690|        }
   36|    690|    }
   37|       |
   38|       |    /// Creates a replacement policy.
   39|       |    #[must_use]
   40|       |    #[inline]
   41|    657|    pub const fn replace(replacement: char) -> Self {
   42|    657|        Self::new(UnmappableAction::Replace, replacement)
   43|    657|    }
   44|       |
   45|       |    /// Creates an ignore policy with the default replacement retained for
   46|       |    /// metadata.
   47|       |    #[must_use]
   48|       |    #[inline]
   49|     13|    pub const fn ignore() -> Self {
   50|     13|        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
   51|     13|    }
   52|       |
   53|       |    /// Creates an ignore policy with explicit replacement metadata.
   54|       |    #[must_use]
   55|       |    #[inline]
   56|     16|    pub const fn ignore_with_replacement(replacement: char) -> Self {
   57|     16|        Self::new(UnmappableAction::Ignore, replacement)
   58|     16|    }
   59|       |
   60|       |    /// Creates a report policy with the default replacement retained for
   61|       |    /// metadata.
   62|       |    #[must_use]
   63|       |    #[inline]
   64|     17|    pub const fn report() -> Self {
   65|     17|        Self::new(UnmappableAction::Report, Self::DEFAULT_REPLACEMENT)
   66|     17|    }
   67|       |
   68|       |    /// Returns the unmappable-input action.
   69|       |    #[must_use]
   70|       |    #[inline]
   71|  1.35k|    pub const fn unmappable_action(self) -> UnmappableAction {
   72|  1.35k|        self.unmappable_action
   73|  1.35k|    }
   74|       |
   75|       |    /// Returns the replacement character.
   76|       |    #[must_use]
   77|       |    #[inline]
   78|  1.32k|    pub const fn replacement(self) -> char {
   79|  1.32k|        self.replacement
   80|  1.32k|    }
   81|       |
   82|       |    /// Returns the default replacement policy supported by `codec`.
   83|       |    ///
   84|       |    /// # Parameters
   85|       |    ///
   86|       |    /// - `codec`: Codec used to check replacement encodability.
   87|       |    ///
   88|       |    /// # Returns
   89|       |    ///
   90|       |    /// Returns the default replacement policy when `U+FFFD` is encodable, or
   91|       |    /// the fallback `?` policy when only the fallback is encodable.
   92|       |    ///
   93|       |    /// # Errors
   94|       |    ///
   95|       |    /// Returns [`CharsetEncodeError`] when neither replacement can be encoded
   96|       |    /// by `codec`.
   97|    558|    pub fn default_for<C>(codec: &C) -> Result<Self, CharsetEncodeError>
   98|    558|    where
   99|    558|        C: CharsetCodec,
  100|       |    {
  101|    558|        if codec.can_encode_value(&Self::DEFAULT_REPLACEMENT) {
  102|    535|            return Ok(Self::default());
  103|     23|        }
  104|     23|        if codec.can_encode_value(&Self::DEFAULT_FALLBACK_REPLACEMENT) {
  105|     20|            return Ok(Self::replace(Self::DEFAULT_FALLBACK_REPLACEMENT));
  106|      3|        }
  107|      3|        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
  108|      3|            value: Self::DEFAULT_FALLBACK_REPLACEMENT as u32,
  109|      3|        };
  110|      3|        Err(CharsetEncodeError::new(codec.charset(), kind, 0))
  111|    558|    }
  112|       |}
  113|       |
  114|       |impl Default for CharsetEncodePolicy {
  115|       |    #[inline]
  116|    613|    fn default() -> Self {
  117|    613|        Self::replace(Self::DEFAULT_REPLACEMENT)
  118|    613|    }
  119|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/encode/charset_encoder.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use qubit_codec::CapacityError;
    9|       |use qubit_codec::TranscodeEncodeErrorOf;
   10|       |use qubit_codec::TranscodeEncoder;
   11|       |use qubit_codec::TranscodeProgress;
   12|       |use qubit_codec::Transcoder;
   13|       |use qubit_codec::engine::TranscodeEncodeEngine;
   14|       |
   15|       |use super::charset_encode_hooks::CharsetEncodeHooks;
   16|       |use super::charset_encode_hooks::replacement_len;
   17|       |use super::charset_encode_policy::CharsetEncodePolicy;
   18|       |use crate::CharsetCodec;
   19|       |use crate::CharsetEncodeError;
   20|       |use crate::UnmappableAction;
   21|       |
   22|       |/// Converts Unicode scalar values into units of one charset.
   23|       |///
   24|       |/// `CharsetEncoder` wraps a low-level [`crate::CharsetCodec`] and applies the
   25|       |/// configured [`UnmappableAction`] whenever the codec reports that an input
   26|       |/// character cannot be represented by the target charset.
   27|       |///
   28|       |/// # Type Parameters
   29|       |///
   30|       |/// - `C`: Low-level charset codec used to encode one character into target
   31|       |///   storage units.
   32|       |#[derive(Debug)]
   33|       |pub struct CharsetEncoder<C>
   34|       |where
   35|       |    C: CharsetCodec,
   36|       |{
   37|       |    /// Common buffered encode engine.
   38|       |    engine: TranscodeEncodeEngine<C, CharsetEncodeHooks>,
   39|       |    /// Public unmappable-input policy metadata.
   40|       |    policy: CharsetEncodePolicy,
   41|       |}
   42|       |
   43|       |impl<C> CharsetEncoder<C>
   44|       |where
   45|       |    C: CharsetCodec,
   46|       |{
   47|       |    /// Creates an encoder with default replacement policy.
   48|       |    ///
   49|       |    /// # Parameters
   50|       |    ///
   51|       |    /// - `codec`: Low-level charset codec used to encode output units.
   52|       |    ///
   53|       |    /// # Returns
   54|       |    ///
   55|       |    /// Returns an encoder whose unmappable action is
   56|       |    /// [`UnmappableAction::Replace`] and whose replacement character is
   57|       |    /// [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`]. If the default cannot be
   58|       |    /// encoded by the codec,
   59|       |    /// [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] is used.
   60|       |    ///
   61|       |    /// # Panics
   62|       |    ///
   63|       |    /// Panics when neither [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`] nor
   64|       |    /// [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] can be encoded by
   65|       |    /// `codec`. This panic is intentional by design: reaching this branch means
   66|       |    /// the supplied codec implementation is wrong, because the API requires a
   67|       |    /// default replacement fallback that the codec can encode. Built-in codecs
   68|       |    /// can always encode the fallback `?`; custom [`crate::CharsetCodec`]
   69|       |    /// implementations that cannot encode it violate the replacement
   70|       |    /// invariant and must fail fast during construction rather than defer the
   71|       |    /// bug to user input.
   72|       |    #[must_use]
   73|    532|    pub fn new(codec: C) -> Self {
   74|    532|        let policy = CharsetEncodePolicy::default_for(&codec).unwrap_or_else(|error| {
                                                                                                   ^1
   75|       |            // This panic is intentional. If default replacement selection gets
   76|       |            // here, the codec cannot encode even the required fallback `?`.
   77|       |            // That violates the codec invariant expected by this API, so
   78|       |            // construction fails fast to expose the broken codec implementation.
   79|      1|            panic!(
   80|       |                "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({error})",
   81|      1|                codec.charset(),
   82|       |                CharsetEncodePolicy::DEFAULT_REPLACEMENT,
   83|       |                CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
   84|       |            )
   85|       |        });
   86|    532|        let hooks = Self::create_hooks(&codec, policy)
   87|       |            // A policy chosen by `default_for` must be encodable; failing here
   88|       |            // can only mean the codec violates the replacement fallback
   89|       |            // invariant. This panic is deliberate for the same fail-fast reason
   90|       |            // as the default-policy panic above.
   91|    532|            .expect(
   92|    532|                "validated default encode policy should create hooks; \
   93|    532|                 failure means the codec violated its replacement invariant",
   94|       |            );
   95|    532|        Self {
   96|    532|            engine: TranscodeEncodeEngine::new(codec, hooks),
   97|    532|            policy,
   98|    532|        }
   99|    532|    }
  100|       |
  101|       |    /// Creates an encoder with an explicit unmappable-input policy.
  102|       |    ///
  103|       |    /// # Errors
  104|       |    ///
  105|       |    /// Returns an error when `policy` uses replacement and the replacement
  106|       |    /// character cannot be encoded by `codec`.
  107|     35|    pub fn with_policy(codec: C, policy: CharsetEncodePolicy) -> Result<Self, CharsetEncodeError> {
  108|     35|        let hooks = Self::create_hooks(&codec, policy)?;
                          ^33                                       ^2
  109|     33|        Ok(Self {
  110|     33|            engine: TranscodeEncodeEngine::new(codec, hooks),
  111|     33|            policy,
  112|     33|        })
  113|     35|    }
  114|       |
  115|       |    /// Returns the configured unmappable-character action.
  116|       |    ///
  117|       |    /// # Returns
  118|       |    ///
  119|       |    /// Returns the action used when target encoding cannot represent a
  120|       |    /// character.
  121|       |    #[inline(always)]
  122|       |    #[must_use]
  123|      3|    pub const fn unmappable_action(&self) -> UnmappableAction {
  124|      3|        self.policy.unmappable_action()
  125|      3|    }
  126|       |
  127|       |    /// Returns the configured replacement character.
  128|       |    ///
  129|       |    /// # Returns
  130|       |    ///
  131|       |    /// Returns the character encoded when [`UnmappableAction::Replace`] is
  132|       |    /// used.
  133|       |    #[inline(always)]
  134|       |    #[must_use]
  135|      4|    pub const fn replacement(&self) -> char {
  136|      4|        self.policy.replacement()
  137|      4|    }
  138|       |
  139|       |    /// Returns the charset encoded by the wrapped codec.
  140|       |    ///
  141|       |    /// # Returns
  142|       |    ///
  143|       |    /// Returns the charset reported by the low-level codec.
  144|       |    #[inline(always)]
  145|       |    #[must_use]
  146|      2|    pub fn charset(&self) -> crate::Charset {
  147|      2|        self.codec().charset()
  148|      2|    }
  149|       |
  150|       |    /// Returns the wrapped low-level codec.
  151|       |    ///
  152|       |    /// # Returns
  153|       |    ///
  154|       |    /// Returns a shared reference to the codec owned by this encoder.
  155|       |    #[inline(always)]
  156|       |    #[must_use]
  157|      3|    pub fn codec(&self) -> &C {
  158|      3|        self.engine.codec()
  159|      3|    }
  160|       |
  161|       |    /// Returns the wrapped low-level codec mutably.
  162|       |    ///
  163|       |    /// Mutating codec configuration while a stream is active can invalidate
  164|       |    /// buffered encode state and replacement validation assumptions. Reset the
  165|       |    /// encoder before continuing with the modified codec.
  166|       |    ///
  167|       |    /// # Returns
  168|       |    ///
  169|       |    /// Returns a mutable reference to the codec owned by this encoder.
  170|       |    #[inline(always)]
  171|       |    #[must_use]
  172|      1|    pub fn codec_mut(&mut self) -> &mut C {
  173|      1|        self.engine.codec_mut()
  174|      1|    }
  175|       |
  176|       |    /// Consumes the encoder and returns its codec.
  177|       |    ///
  178|       |    /// Encoder policy and hook state are discarded.
  179|       |    ///
  180|       |    /// # Returns
  181|       |    ///
  182|       |    /// Returns the low-level codec owned by this encoder.
  183|       |    #[inline(always)]
  184|       |    #[must_use]
  185|      1|    pub fn into_codec(self) -> C {
  186|      1|        let (codec, _) = self.engine.into_parts();
  187|      1|        codec
  188|      1|    }
  189|       |
  190|       |    /// Runs encoder reset.
  191|       |    ///
  192|       |    /// # Errors
  193|       |    ///
  194|       |    /// Returns a transcode encode error when `output_index` is invalid, output
  195|       |    /// capacity is insufficient, or encoder reset emits a charset-domain
  196|       |    /// error.
  197|       |    #[inline]
  198|     14|    pub fn reset(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, TranscodeEncodeErrorOf<C>> {
  199|     14|        self.engine.reset(output, output_index)
  200|     14|    }
  201|       |
  202|       |    /// Encodes characters into target units.
  203|       |    ///
  204|       |    /// # Errors
  205|       |    ///
  206|       |    /// Returns a transcode encode error when indices are invalid, output
  207|       |    /// capacity is insufficient, a character is unmappable under the
  208|       |    /// configured policy, or the codec reports another encode-domain error.
  209|       |    #[inline]
  210|     53|    pub fn transcode(
  211|     53|        &mut self,
  212|     53|        input: &[char],
  213|     53|        input_index: usize,
  214|     53|        output: &mut [C::Unit],
  215|     53|        output_index: usize,
  216|     53|    ) -> Result<TranscodeProgress, TranscodeEncodeErrorOf<C>> {
  217|     53|        self.engine.transcode(input, input_index, output, output_index)
  218|     53|    }
  219|       |
  220|       |    /// Maps an encoder transcode error into a charset encode error.
  221|       |    ///
  222|       |    /// Framework failures and unencodable values are attached to this
  223|       |    /// encoder's charset. Codec-domain errors retain their original charset
  224|       |    /// error.
  225|       |    #[must_use]
  226|       |    #[inline]
  227|      1|    pub fn map_transcode_error(&self, error: TranscodeEncodeErrorOf<C>) -> CharsetEncodeError {
  228|      1|        CharsetEncodeError::from_transcode_error(self.charset(), error)
  229|      1|    }
  230|       |
  231|       |    /// Finishes encoder-owned final output after EOF.
  232|       |    ///
  233|       |    /// # Errors
  234|       |    ///
  235|       |    /// Returns a transcode encode error when finalization output cannot be
  236|       |    /// written or when the codec reports a final encode-domain error.
  237|       |    #[inline]
  238|     22|    pub fn finish(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, TranscodeEncodeErrorOf<C>> {
  239|     22|        self.engine.finish(output, output_index)
  240|     22|    }
  241|       |
  242|       |    /// Runs a complete `reset -> transcode -> finish` encode stream.
  243|       |    ///
  244|       |    /// # Errors
  245|       |    ///
  246|       |    /// Returns a transcode encode error when the supplied output buffer is too
  247|       |    /// small, a character cannot be represented under the configured policy,
  248|       |    /// or the codec reports a charset-domain encode error.
  249|       |    #[inline]
  250|    517|    pub fn transcode_complete_into(
  251|    517|        &mut self,
  252|    517|        input: &[char],
  253|    517|        output: &mut [C::Unit],
  254|    517|    ) -> Result<usize, TranscodeEncodeErrorOf<C>> {
  255|    517|        <Self as Transcoder>::transcode_complete_into(self, input, output)
  256|    517|    }
  257|       |
  258|       |    /// Creates encode hooks for `policy`.
  259|    671|    pub(crate) fn create_hooks(
  260|    671|        codec: &C,
  261|    671|        policy: CharsetEncodePolicy,
  262|    671|    ) -> Result<CharsetEncodeHooks, CharsetEncodeError> {
  263|    671|        let hooks = CharsetEncodeHooks::new(policy.unmappable_action(), policy.replacement());
  264|    671|        if policy.unmappable_action() != UnmappableAction::Replace {
  265|     25|            return Ok(hooks);
  266|    646|        }
  267|    646|        replacement_len(codec, policy.replacement())?;
                                                                  ^3
  268|    643|        Ok(hooks)
  269|    671|    }
  270|       |}
  271|       |
  272|       |impl<C> Transcoder for CharsetEncoder<C>
  273|       |where
  274|       |    C: CharsetCodec,
  275|       |{
  276|       |    type Input = char;
  277|       |    type Output = C::Unit;
  278|       |    type Error = TranscodeEncodeErrorOf<C>;
  279|       |
  280|       |    /// Returns the maximum number of target units needed for `input_len`
  281|       |    /// characters.
  282|       |    #[inline(always)]
  283|  1.03k|    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
  284|  1.03k|        self.engine.max_transcode_output_len(input_len)
  285|  1.03k|    }
  286|       |
  287|       |    /// Returns the maximum target units emitted by finishing internal state.
  288|       |    #[inline(always)]
  289|  1.03k|    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
  290|  1.03k|        self.engine.max_finish_output_len()
  291|  1.03k|    }
  292|       |
  293|       |    /// Returns the maximum target units emitted when resetting stream state.
  294|       |    #[inline(always)]
  295|  1.03k|    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
  296|  1.03k|        self.engine.max_reset_output_len()
  297|  1.03k|    }
  298|       |
  299|       |    /// Runs encoder reset while keeping encoder policy.
  300|       |    #[inline(always)]
  301|    557|    fn reset(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, Self::Error> {
  302|    557|        self.engine.reset(output, output_index)
  303|    557|    }
  304|       |
  305|       |    /// Encodes characters into the target charset while applying unmappable
  306|       |    /// policy.
  307|       |    #[inline(always)]
  308|    517|    fn transcode(
  309|    517|        &mut self,
  310|    517|        input: &[char],
  311|    517|        input_index: usize,
  312|    517|        output: &mut [C::Unit],
  313|    517|        output_index: usize,
  314|    517|    ) -> Result<TranscodeProgress, Self::Error> {
  315|    517|        self.engine.transcode(input, input_index, output, output_index)
  316|    517|    }
  317|       |
  318|       |    /// Finishes encoder-owned final output after EOF.
  319|       |    #[inline(always)]
  320|    516|    fn finish(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, Self::Error> {
  321|    516|        self.engine.finish(output, output_index)
  322|    516|    }
  323|       |}
  324|       |
  325|       |impl<C> TranscodeEncoder for CharsetEncoder<C>
  326|       |where
  327|       |    C: CharsetCodec,
  328|       |{
  329|       |    type EncodeError = CharsetEncodeError;
  330|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/error/charset_decode_error.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::error::Error;
    9|       |use core::fmt;
   10|       |use core::num::NonZeroUsize;
   11|       |
   12|       |use qubit_codec::DecodeFailure;
   13|       |use qubit_codec::TranscodeDecodeError;
   14|       |use qubit_codec::TranscodeFailure;
   15|       |
   16|       |use crate::Charset;
   17|       |use crate::CharsetDecodeErrorKind;
   18|       |
   19|       |/// Error reported by a charset decoder.
   20|       |///
   21|       |/// The error always carries the charset, error kind, and input unit index at
   22|       |/// which the failure was detected. Errors that decode a raw numeric value, such
   23|       |/// as malformed input units or invalid UTF-32 scalar values, carry that value
   24|       |/// through [`Self::kind`] and [`Self::value`]. Invalid-input errors may also
   25|       |/// carry a consumed-unit count so buffered decoders can make progress without
   26|       |/// an extra status wrapper. Buffer index and output-capacity failures are
   27|       |/// represented by dedicated [`CharsetDecodeErrorKind`] variants so the public
   28|       |/// decoder API can expose a single domain error type.
   29|       |#[derive(Clone, Copy, Debug, Eq, PartialEq)]
   30|       |pub struct CharsetDecodeError {
   31|       |    /// Charset being decoded when this error was detected.
   32|       |    charset: Charset,
   33|       |    /// Failure category describing the decoding error.
   34|       |    kind: CharsetDecodeErrorKind,
   35|       |    /// Input unit index at which decoding failure occurred.
   36|       |    index: usize,
   37|       |    /// Units that may be consumed after invalid input.
   38|       |    consumed: Option<NonZeroUsize>,
   39|       |}
   40|       |
   41|       |/// Result type returned by charset decoders.
   42|       |///
   43|       |/// # Type Parameters
   44|       |///
   45|       |/// - `T`: Successful value produced by a decoding operation.
   46|       |pub type CharsetDecodeResult<T> = Result<T, CharsetDecodeError>;
   47|       |
   48|       |/// Result type returned by [`qubit_codec::Codec`] charset decoders.
   49|       |pub(crate) type CharsetCodecDecodeResult<T> = Result<T, DecodeFailure<CharsetDecodeError>>;
   50|       |
   51|       |impl CharsetDecodeError {
   52|       |    /// Maps a decoder transcode error into a charset decode error.
   53|       |    ///
   54|       |    /// Framework failures are mapped with `charset`; codec-domain errors retain
   55|       |    /// their original charset error.
   56|       |    #[must_use]
   57|      2|    pub fn from_transcode_error(charset: Charset, error: TranscodeDecodeError<Self>) -> Self {
   58|      2|        match error {
   59|      1|            TranscodeDecodeError::Failure(failure) => Self::map_transcode_failure(charset, failure),
   60|      1|            TranscodeDecodeError::Domain(error) => error.into_source(),
   61|       |        }
   62|      2|    }
   63|       |
   64|       |    /// Maps a transcode-layer failure into a charset decode error.
   65|       |    ///
   66|       |    /// # Parameters
   67|       |    ///
   68|       |    /// - `charset`: Charset being decoded.
   69|       |    /// - `error`: Framework failure reported by the transcode layer.
   70|       |    ///
   71|       |    /// # Returns
   72|       |    ///
   73|       |    /// Returns the charset-level representation of `error`. Buffer and index
   74|       |    /// failures retain their original indices and sizes. Encode-only framework
   75|       |    /// failures are reported as
   76|       |    /// [`CharsetDecodeErrorKind::UnexpectedTranscodeFailure`].
   77|       |    #[must_use]
   78|     21|    pub fn map_transcode_failure(charset: Charset, error: TranscodeFailure) -> Self {
   79|       |        use TranscodeFailure::IncompleteInput;
   80|       |        use TranscodeFailure::InsufficientOutput;
   81|       |        use TranscodeFailure::InvalidInputIndex;
   82|       |        use TranscodeFailure::InvalidOutputIndex;
   83|       |        use TranscodeFailure::OutputLengthOverflow;
   84|       |
   85|     21|        match error {
   86|      5|            InvalidInputIndex { index, input_len } => {
   87|      5|                Self::new(charset, CharsetDecodeErrorKind::InvalidInputIndex { input_len }, index)
   88|       |            }
   89|      4|            InvalidOutputIndex { index, output_len } => Self::new(
   90|      4|                charset,
   91|      4|                CharsetDecodeErrorKind::InvalidOutputIndex { output_len },
   92|      4|                index,
   93|       |            ),
   94|       |            InsufficientOutput {
   95|      1|                output_index,
   96|      1|                required,
   97|      1|                available,
   98|      1|            } => Self::new(
   99|      1|                charset,
  100|      1|                CharsetDecodeErrorKind::BufferTooSmall { required, available },
  101|      1|                output_index,
  102|       |            ),
  103|      1|            OutputLengthOverflow => Self::new(charset, CharsetDecodeErrorKind::OutputLengthOverflow, usize::MAX),
  104|       |            IncompleteInput {
  105|      4|                input_index,
  106|      4|                required,
  107|      4|                available,
  108|      4|            } => Self::new(
  109|      4|                charset,
  110|      4|                CharsetDecodeErrorKind::IncompleteSequence { required, available },
  111|      4|                input_index,
  112|       |            ),
  113|      6|            _ => Self::new(charset, CharsetDecodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
  114|       |        }
  115|     21|    }
  116|       |
  117|       |    /// Creates a decoding error.
  118|       |    ///
  119|       |    /// # Parameters
  120|       |    ///
  121|       |    /// - `charset`: The charset being decoded.
  122|       |    /// - `kind`: The failure category.
  123|       |    /// - `index`: The input unit index where the failure was detected.
  124|       |    ///
  125|       |    /// # Returns
  126|       |    ///
  127|       |    /// Returns a decoding error carrying the supplied context.
  128|       |    #[inline]
  129|  26.3k|    pub const fn new(charset: Charset, kind: CharsetDecodeErrorKind, index: usize) -> Self {
  130|       |        Self {
  131|  26.3k|            charset,
  132|  26.3k|            kind,
  133|  26.3k|            index,
  134|  26.3k|            consumed: match kind {
  135|       |                CharsetDecodeErrorKind::MalformedSequence { .. } | CharsetDecodeErrorKind::InvalidCodePoint { .. } => {
  136|  26.1k|                    Some(NonZeroUsize::MIN)
  137|       |                }
  138|       |                CharsetDecodeErrorKind::InvalidInputIndex { .. }
  139|       |                | CharsetDecodeErrorKind::InvalidOutputIndex { .. }
  140|       |                | CharsetDecodeErrorKind::BufferTooSmall { .. }
  141|       |                | CharsetDecodeErrorKind::OutputLengthOverflow
  142|       |                | CharsetDecodeErrorKind::UnexpectedTranscodeFailure
  143|    123|                | CharsetDecodeErrorKind::IncompleteSequence { .. } => None,
  144|       |            },
  145|       |        }
  146|  26.3k|    }
  147|       |
  148|       |    /// Returns a copy of this error with invalid-input consumption context.
  149|       |    ///
  150|       |    /// # Parameters
  151|       |    ///
  152|       |    /// - `consumed`: Number of units that may be consumed to make progress.
  153|       |    ///
  154|       |    /// # Returns
  155|       |    ///
  156|       |    /// Returns this error carrying the supplied consumption count.
  157|       |    #[must_use]
  158|       |    #[inline]
  159|  10.2k|    pub const fn with_consumed(self, consumed: NonZeroUsize) -> Self {
  160|  10.2k|        Self {
  161|  10.2k|            charset: self.charset,
  162|  10.2k|            kind: self.kind,
  163|  10.2k|            index: self.index,
  164|  10.2k|            consumed: Some(consumed),
  165|  10.2k|        }
  166|  10.2k|    }
  167|       |
  168|       |    /// Converts this error into the failure type returned by
  169|       |    /// [`qubit_codec::Codec::decode`].
  170|       |    #[inline]
  171|  26.2k|    pub fn into_codec_failure(self) -> DecodeFailure<Self> {
  172|  26.2k|        if let Some((required, _)) = self.kind.incomplete() {
                                   ^96
  173|     96|            if let Some(required) = NonZeroUsize::new(required) {
                                      ^95
  174|     95|                DecodeFailure::incomplete_with_source(self, required)
  175|       |            } else {
  176|       |                #[cfg(debug_assertions)]
  177|      1|                panic!("incomplete charset decode errors must require non-zero units",);
  178|       |                #[cfg(not(debug_assertions))]
  179|       |                {
  180|       |                    DecodeFailure::invalid_unknown(self)
  181|       |                }
  182|       |            }
  183|  26.1k|        } else if let Some(consumed) = self.consumed() {
                                         ^26.1k
  184|  26.1k|            DecodeFailure::invalid(self, consumed)
  185|       |        } else {
  186|      2|            DecodeFailure::invalid_unknown(self)
  187|       |        }
  188|  26.2k|    }
  189|       |
  190|       |    /// Returns the charset being decoded.
  191|       |    ///
  192|       |    /// # Returns
  193|       |    ///
  194|       |    /// Returns the stored [`Charset`].
  195|       |    #[inline]
  196|      5|    pub const fn charset(self) -> Charset {
  197|      5|        self.charset
  198|      5|    }
  199|       |
  200|       |    /// Returns the decoding error kind.
  201|       |    ///
  202|       |    /// # Returns
  203|       |    ///
  204|       |    /// Returns the stored [`CharsetDecodeErrorKind`].
  205|       |    #[inline]
  206|  26.2k|    pub const fn kind(self) -> CharsetDecodeErrorKind {
  207|  26.2k|        self.kind
  208|  26.2k|    }
  209|       |
  210|       |    /// Returns the input unit index associated with this error.
  211|       |    ///
  212|       |    /// # Returns
  213|       |    ///
  214|       |    /// Returns the index at which the error was detected.
  215|       |    #[inline]
  216|     62|    pub const fn index(self) -> usize {
  217|     62|        self.index
  218|     62|    }
  219|       |
  220|       |    /// Returns required input units for this decoding error, if reported.
  221|       |    ///
  222|       |    /// # Returns
  223|       |    ///
  224|       |    /// Returns `Some(required)` for
  225|       |    /// [`CharsetDecodeErrorKind::IncompleteSequence`] and
  226|       |    /// [`CharsetDecodeErrorKind::BufferTooSmall`], otherwise `None`.
  227|       |    #[inline]
  228|      4|    pub const fn required(self) -> Option<usize> {
  229|      4|        self.kind.required()
  230|      4|    }
  231|       |
  232|       |    /// Returns available input units for this decoding error, if reported.
  233|       |    ///
  234|       |    /// # Returns
  235|       |    ///
  236|       |    /// Returns `Some(available)` for
  237|       |    /// [`CharsetDecodeErrorKind::IncompleteSequence`] and
  238|       |    /// [`CharsetDecodeErrorKind::BufferTooSmall`], otherwise `None`.
  239|       |    #[inline]
  240|      3|    pub const fn available(self) -> Option<usize> {
  241|      3|        self.kind.available()
  242|      3|    }
  243|       |
  244|       |    /// Returns the raw value associated with this error.
  245|       |    ///
  246|       |    /// # Returns
  247|       |    ///
  248|       |    /// Returns `Some(value)` when the error kind carries a raw unit or code
  249|       |    /// point value, or `None` for kinds without an associated value.
  250|       |    #[inline]
  251|      6|    pub const fn value(self) -> Option<u32> {
  252|      6|        self.kind.value()
  253|      6|    }
  254|       |
  255|       |    /// Returns units that may be consumed after this invalid-input error.
  256|       |    ///
  257|       |    /// # Returns
  258|       |    ///
  259|       |    /// Returns `Some(consumed)` for malformed and invalid-code-point input, or
  260|       |    /// `None` for incomplete input.
  261|       |    #[inline]
  262|  52.3k|    pub const fn consumed(self) -> Option<NonZeroUsize> {
  263|  52.3k|        self.consumed
  264|  52.3k|    }
  265|       |
  266|       |    /// Offsets this error by a base unit index.
  267|       |    ///
  268|       |    /// # Parameters
  269|       |    ///
  270|       |    /// - `base`: The base index to add to the stored index.
  271|       |    ///
  272|       |    /// # Returns
  273|       |    ///
  274|       |    /// Returns a copy of this error with its index shifted by `base`.
  275|       |    ///
  276|       |    /// If the shifted index cannot be represented, it is saturated to
  277|       |    /// [`usize::MAX`].
  278|       |    #[inline]
  279|      2|    pub const fn offset_by(self, base: usize) -> Self {
  280|       |        Self {
  281|      2|            charset: self.charset,
  282|      2|            kind: self.kind,
  283|      2|            index: match self.index.checked_add(base) {
  284|      1|                Some(index) => index,
  285|      1|                None => usize::MAX,
  286|       |            },
  287|      2|            consumed: self.consumed,
  288|       |        }
  289|      2|    }
  290|       |}
  291|       |
  292|       |impl fmt::Display for CharsetDecodeError {
  293|       |    /// Formats this decoding error.
  294|       |    ///
  295|       |    /// # Parameters
  296|       |    ///
  297|       |    /// - `formatter`: The formatter receiving the diagnostic message.
  298|       |    ///
  299|       |    /// # Errors
  300|       |    ///
  301|       |    /// Returns any formatting error reported by `formatter`.
  302|      4|    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
  303|      4|        if let Some(value) = self.kind.value() {
                                  ^1
  304|      1|            write!(
  305|      1|                formatter,
  306|       |                "{} decoding error at index {} for value 0x{:x}: {}",
  307|       |                self.charset, self.index, value, self.kind,
  308|       |            )
  309|       |        } else {
  310|      3|            write!(
  311|      3|                formatter,
  312|       |                "{} decoding error at index {}: {}",
  313|       |                self.charset, self.index, self.kind,
  314|       |            )
  315|       |        }
  316|      4|    }
  317|       |}
  318|       |
  319|       |impl Error for CharsetDecodeError {}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/error/charset_decode_error_kind.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use thiserror::Error;
    9|       |
   10|       |/// Classifies failures detected while decoding encoded units into Unicode text.
   11|       |#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
   12|       |#[non_exhaustive]
   13|       |pub enum CharsetDecodeErrorKind {
   14|       |    /// The requested input unit index is outside the input buffer.
   15|       |    #[error("The input unit index is outside the input buffer.")]
   16|       |    InvalidInputIndex {
   17|       |        /// Length of the input provided to the codec call.
   18|       |        input_len: usize,
   19|       |    },
   20|       |
   21|       |    /// The requested output character index is outside the output buffer.
   22|       |    #[error("The output character index is outside the output buffer.")]
   23|       |    InvalidOutputIndex {
   24|       |        /// Length of the output provided to the codec call.
   25|       |        output_len: usize,
   26|       |    },
   27|       |
   28|       |    /// The supplied output buffer is too small for decoded characters.
   29|       |    #[error("The output buffer is too small (required {required} characters, available {available} characters).")]
   30|       |    BufferTooSmall {
   31|       |        /// Total output characters required.
   32|       |        required: usize,
   33|       |
   34|       |        /// Total output characters currently available.
   35|       |        available: usize,
   36|       |    },
   37|       |
   38|       |    /// Output length arithmetic overflowed.
   39|       |    #[error("The output length arithmetic overflowed.")]
   40|       |    OutputLengthOverflow,
   41|       |
   42|       |    /// The transcode layer reported a framework failure that has no
   43|       |    /// charset-specific decode representation.
   44|       |    #[error("The transcode layer reported an unexpected framework failure.")]
   45|       |    UnexpectedTranscodeFailure,
   46|       |
   47|       |    /// The input units do not form a well-formed encoded sequence.
   48|       |    #[error("The encoded text sequence is malformed.")]
   49|       |    MalformedSequence {
   50|       |        /// Optional malformed raw value captured from the offending input
   51|       |        /// unit.
   52|       |        value: Option<u32>,
   53|       |    },
   54|       |
   55|       |    /// The closed input ended before a complete character was available.
   56|       |    #[error("The encoded text sequence is incomplete (required {required} units, available {available} units).")]
   57|       |    IncompleteSequence {
   58|       |        /// Total units required to complete the current sequence.
   59|       |        required: usize,
   60|       |
   61|       |        /// Total units currently available from the start of the current
   62|       |        /// sequence.
   63|       |        available: usize,
   64|       |    },
   65|       |
   66|       |    /// The decoded numeric value is not a valid Unicode scalar value.
   67|       |    #[error("The decoded code point 0x{value:x} is not a valid Unicode scalar value.")]
   68|       |    InvalidCodePoint {
   69|       |        /// Raw decoded code-point value.
   70|       |        value: u32,
   71|       |    },
   72|       |}
   73|       |
   74|       |impl CharsetDecodeErrorKind {
   75|       |    /// Creates a malformed-sequence error with a captured raw value.
   76|       |    ///
   77|       |    /// # Parameters
   78|       |    ///
   79|       |    /// - `value`: Raw malformed unit or byte value observed by the decoder.
   80|       |    ///
   81|       |    /// # Returns
   82|       |    ///
   83|       |    /// Returns [`Self::MalformedSequence`] carrying `value`.
   84|       |    #[must_use]
   85|       |    #[inline]
   86|  26.1k|    pub const fn malformed(value: u32) -> Self {
   87|  26.1k|        Self::MalformedSequence { value: Some(value) }
   88|  26.1k|    }
   89|       |
   90|       |    /// Creates a malformed-sequence error without a captured raw value.
   91|       |    ///
   92|       |    /// # Returns
   93|       |    ///
   94|       |    /// Returns [`Self::MalformedSequence`] with no associated raw value.
   95|       |    #[must_use]
   96|       |    #[inline]
   97|     21|    pub const fn malformed_unknown() -> Self {
   98|     21|        Self::MalformedSequence { value: None }
   99|     21|    }
  100|       |
  101|       |    /// Returns the number of required input units for this kind, if available.
  102|       |    ///
  103|       |    /// # Returns
  104|       |    ///
  105|       |    /// - `Some(required)` for [`Self::IncompleteSequence`] and
  106|       |    ///   [`Self::BufferTooSmall`];
  107|       |    /// - `None` for all other variants.
  108|       |    #[must_use]
  109|       |    #[inline]
  110|      8|    pub const fn required(self) -> Option<usize> {
  111|      8|        match self {
  112|      4|            Self::IncompleteSequence { required, .. } => Some(required),
  113|      1|            Self::BufferTooSmall { required, .. } => Some(required),
  114|       |            Self::InvalidInputIndex { .. }
  115|       |            | Self::InvalidOutputIndex { .. }
  116|       |            | Self::OutputLengthOverflow
  117|       |            | Self::UnexpectedTranscodeFailure
  118|       |            | Self::MalformedSequence { .. }
  119|      3|            | Self::InvalidCodePoint { .. } => None,
  120|       |        }
  121|      8|    }
  122|       |
  123|       |    /// Returns the number of currently available input units for this kind, if
  124|       |    /// available.
  125|       |    ///
  126|       |    /// # Returns
  127|       |    ///
  128|       |    /// - `Some(available)` for [`Self::IncompleteSequence`] and
  129|       |    ///   [`Self::BufferTooSmall`];
  130|       |    /// - `None` for all other variants.
  131|       |    #[must_use]
  132|       |    #[inline]
  133|      7|    pub const fn available(self) -> Option<usize> {
  134|      7|        match self {
  135|      4|            Self::IncompleteSequence { available, .. } => Some(available),
  136|      1|            Self::BufferTooSmall { available, .. } => Some(available),
  137|       |            Self::InvalidInputIndex { .. }
  138|       |            | Self::InvalidOutputIndex { .. }
  139|       |            | Self::OutputLengthOverflow
  140|       |            | Self::UnexpectedTranscodeFailure
  141|       |            | Self::MalformedSequence { .. }
  142|      2|            | Self::InvalidCodePoint { .. } => None,
  143|       |        }
  144|      7|    }
  145|       |
  146|       |    /// Returns the raw malformed value associated with this decoding error, if
  147|       |    /// any.
  148|       |    ///
  149|       |    /// # Returns
  150|       |    ///
  151|       |    /// - `Some(value)` for [`Self::MalformedSequence`] when a specific
  152|       |    ///   malformed unit value is available.
  153|       |    /// - `Some(value)` for [`Self::InvalidCodePoint`].
  154|       |    /// - `None` for other variants.
  155|       |    #[must_use]
  156|       |    #[inline]
  157|     15|    pub const fn value(self) -> Option<u32> {
  158|     15|        match self {
  159|      6|            Self::MalformedSequence { value } => value,
  160|      6|            Self::InvalidCodePoint { value } => Some(value),
  161|       |            Self::InvalidInputIndex { .. }
  162|       |            | Self::InvalidOutputIndex { .. }
  163|       |            | Self::BufferTooSmall { .. }
  164|       |            | Self::OutputLengthOverflow
  165|       |            | Self::UnexpectedTranscodeFailure
  166|      3|            | Self::IncompleteSequence { .. } => None,
  167|       |        }
  168|     15|    }
  169|       |
  170|       |    /// Returns whether this kind represents incomplete input.
  171|       |    ///
  172|       |    /// # Returns
  173|       |    ///
  174|       |    /// Returns `true` for [`Self::IncompleteSequence`].
  175|       |    #[must_use]
  176|       |    #[inline]
  177|      3|    pub const fn is_incomplete(self) -> bool {
  178|      3|        matches!(self, Self::IncompleteSequence { .. })
                      ^2
  179|      3|    }
  180|       |
  181|       |    /// Returns incomplete-input details for this kind.
  182|       |    ///
  183|       |    /// # Returns
  184|       |    ///
  185|       |    /// Returns `Some((required, available))` for [`Self::IncompleteSequence`],
  186|       |    /// or `None` for all other variants.
  187|       |    #[must_use]
  188|       |    #[inline]
  189|  26.2k|    pub const fn incomplete(self) -> Option<(usize, usize)> {
  190|  26.2k|        match self {
  191|     97|            Self::IncompleteSequence { required, available } => Some((required, available)),
  192|       |            Self::InvalidInputIndex { .. }
  193|       |            | Self::InvalidOutputIndex { .. }
  194|       |            | Self::BufferTooSmall { .. }
  195|       |            | Self::OutputLengthOverflow
  196|       |            | Self::UnexpectedTranscodeFailure
  197|       |            | Self::MalformedSequence { .. }
  198|  26.1k|            | Self::InvalidCodePoint { .. } => None,
  199|       |        }
  200|  26.2k|    }
  201|       |
  202|       |    /// Returns whether this kind represents malformed encoded input.
  203|       |    ///
  204|       |    /// # Returns
  205|       |    ///
  206|       |    /// Returns `true` for malformed sequences and invalid decoded scalar
  207|       |    /// values. Incomplete input is caller-owned tail data and is not
  208|       |    /// treated as malformed input by buffered charset decoders.
  209|       |    #[must_use]
  210|       |    #[inline]
  211|  26.1k|    pub const fn is_malformed_input(self) -> bool {
  212|  26.1k|        matches!(self, Self::MalformedSequence { .. } | Self::InvalidCodePoint { .. })
                      ^2
  213|  26.1k|    }
  214|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/error/charset_encode_error.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use core::error::Error;
    9|       |use core::fmt;
   10|       |
   11|       |use qubit_codec::TranscodeEncodeError;
   12|       |use qubit_codec::TranscodeFailure;
   13|       |
   14|       |use crate::Charset;
   15|       |use crate::CharsetEncodeErrorKind;
   16|       |
   17|       |/// Error reported by a charset encoder.
   18|       |///
   19|       |/// The error always carries the target charset, error kind, and operation
   20|       |/// index associated with the failure. For buffer errors this is the
   21|       |/// caller-supplied output index. Errors tied to a raw code point or character
   22|       |/// value expose that value through [`Self::kind`] and [`Self::value`].
   23|       |#[derive(Clone, Copy, Debug, Eq, PartialEq)]
   24|       |pub struct CharsetEncodeError {
   25|       |    /// Target charset of the operation that produced this error.
   26|       |    charset: Charset,
   27|       |    /// Failure category describing why encoding could not proceed.
   28|       |    kind: CharsetEncodeErrorKind,
   29|       |    /// Output unit index or input code point index where failure occurred.
   30|       |    index: usize,
   31|       |}
   32|       |
   33|       |/// Result type returned by charset encoders.
   34|       |///
   35|       |/// # Type Parameters
   36|       |///
   37|       |/// - `T`: Successful value produced by an encoding operation.
   38|       |pub type CharsetEncodeResult<T> = Result<T, CharsetEncodeError>;
   39|       |
   40|       |impl CharsetEncodeError {
   41|       |    /// Maps an encoder transcode error into a charset encode error.
   42|       |    ///
   43|       |    /// Framework failures and unencodable values are mapped with `charset`;
   44|       |    /// codec-domain errors retain their original charset error.
   45|       |    #[must_use]
   46|      3|    pub fn from_transcode_error(charset: Charset, error: TranscodeEncodeError<Self, char>) -> Self {
   47|      3|        match error {
   48|      1|            TranscodeEncodeError::Failure(failure) => Self::map_transcode_failure(charset, failure),
   49|      1|            TranscodeEncodeError::Unencodable { input_index, value } => {
   50|      1|                Self::map_unencodable(charset, input_index, value)
   51|       |            }
   52|      1|            TranscodeEncodeError::Domain(error) => error.into_source(),
   53|       |        }
   54|      3|    }
   55|       |
   56|       |    /// Maps a transcode-layer failure into a charset encode error.
   57|       |    ///
   58|       |    /// # Parameters
   59|       |    ///
   60|       |    /// - `charset`: Target charset being encoded.
   61|       |    /// - `error`: Framework failure reported by the transcode layer.
   62|       |    ///
   63|       |    /// # Returns
   64|       |    ///
   65|       |    /// Returns the charset-level representation of `error`. Buffer and index
   66|       |    /// failures retain their original indices and sizes.
   67|       |    /// Decode-only framework failures are reported as
   68|       |    /// [`CharsetEncodeErrorKind::UnexpectedTranscodeFailure`] instead of
   69|       |    /// being misreported as output-length overflow.
   70|       |    #[must_use]
   71|     22|    pub fn map_transcode_failure(charset: Charset, error: TranscodeFailure) -> Self {
   72|       |        use TranscodeFailure::IncompleteInput;
   73|       |        use TranscodeFailure::InsufficientOutput;
   74|       |        use TranscodeFailure::InvalidInputIndex;
   75|       |        use TranscodeFailure::InvalidOutputIndex;
   76|       |        use TranscodeFailure::OutputLengthOverflow;
   77|       |        use TranscodeFailure::TrailingInput;
   78|       |
   79|     22|        match error {
   80|      2|            InvalidInputIndex { index, input_len } => {
   81|      2|                Self::new(charset, CharsetEncodeErrorKind::InvalidInputIndex { input_len }, index)
   82|       |            }
   83|      7|            InvalidOutputIndex { index, output_len } => Self::new(
   84|      7|                charset,
   85|      7|                CharsetEncodeErrorKind::InvalidOutputIndex { output_len },
   86|      7|                index,
   87|       |            ),
   88|       |            InsufficientOutput {
   89|      7|                output_index,
   90|      7|                required,
   91|      7|                available,
   92|      7|            } => Self::new(
   93|      7|                charset,
   94|      7|                CharsetEncodeErrorKind::BufferTooSmall { required, available },
   95|      7|                output_index,
   96|       |            ),
   97|      2|            OutputLengthOverflow => Self::new(charset, CharsetEncodeErrorKind::OutputLengthOverflow, usize::MAX),
   98|       |            IncompleteInput {
   99|      1|                input_index,
  100|      1|                required,
  101|      1|                available,
  102|      1|            } => Self::new(
  103|      1|                charset,
  104|      1|                CharsetEncodeErrorKind::IncompleteInput { required, available },
  105|      1|                input_index,
  106|       |            ),
  107|      1|            TrailingInput { .. } => Self::new(charset, CharsetEncodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
  108|      2|            _ => Self::new(charset, CharsetEncodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
  109|       |        }
  110|     22|    }
  111|       |
  112|       |    /// Maps a transcode-layer unencodable value into a charset encode error.
  113|       |    #[must_use]
  114|      4|    pub fn map_unencodable(charset: Charset, input_index: usize, value: Option<char>) -> Self {
  115|      4|        Self::new(
  116|      4|            charset,
  117|      4|            match value {
  118|      3|                Some(value) => CharsetEncodeErrorKind::UnmappableCharacter { value: value as u32 },
  119|      1|                None => CharsetEncodeErrorKind::UnencodableValue,
  120|       |            },
  121|      4|            input_index,
  122|       |        )
  123|      4|    }
  124|       |
  125|       |    /// Creates an encoding error.
  126|       |    ///
  127|       |    /// # Parameters
  128|       |    ///
  129|       |    /// - `charset`: The target charset.
  130|       |    /// - `kind`: The failure category.
  131|       |    /// - `index`: The operation index associated with the failure.
  132|       |    ///
  133|       |    /// # Returns
  134|       |    ///
  135|       |    /// Returns an encoding error carrying the supplied context.
  136|       |    #[inline]
  137|     68|    pub const fn new(charset: Charset, kind: CharsetEncodeErrorKind, index: usize) -> Self {
  138|     68|        Self { charset, kind, index }
  139|     68|    }
  140|       |
  141|       |    /// Returns required output units for this encoding error, if reported.
  142|       |    ///
  143|       |    /// # Returns
  144|       |    ///
  145|       |    /// Returns `Some(required)` for [`CharsetEncodeErrorKind::BufferTooSmall`]
  146|       |    /// and [`CharsetEncodeErrorKind::IncompleteInput`], otherwise `None`.
  147|       |    #[inline]
  148|      2|    pub const fn required(self) -> Option<usize> {
  149|      2|        self.kind.required()
  150|      2|    }
  151|       |
  152|       |    /// Returns available output units for this encoding error, if reported.
  153|       |    ///
  154|       |    /// # Returns
  155|       |    ///
  156|       |    /// Returns `Some(available)` for
  157|       |    /// [`CharsetEncodeErrorKind::BufferTooSmall`] and
  158|       |    /// [`CharsetEncodeErrorKind::IncompleteInput`], otherwise `None`.
  159|       |    #[inline]
  160|      2|    pub const fn available(self) -> Option<usize> {
  161|      2|        self.kind.available()
  162|      2|    }
  163|       |
  164|       |    /// Returns output length for this encoding error, if reported.
  165|       |    ///
  166|       |    /// # Returns
  167|       |    ///
  168|       |    /// Returns `Some(output_len)` for
  169|       |    /// [`CharsetEncodeErrorKind::InvalidOutputIndex`], otherwise `None`.
  170|       |    #[inline]
  171|      2|    pub const fn output_len(self) -> Option<usize> {
  172|      2|        self.kind.output_len()
  173|      2|    }
  174|       |
  175|       |    /// Returns the target charset.
  176|       |    ///
  177|       |    /// # Returns
  178|       |    ///
  179|       |    /// Returns the stored [`Charset`].
  180|       |    #[inline]
  181|      5|    pub const fn charset(self) -> Charset {
  182|      5|        self.charset
  183|      5|    }
  184|       |
  185|       |    /// Returns the encoding error kind.
  186|       |    ///
  187|       |    /// # Returns
  188|       |    ///
  189|       |    /// Returns the stored [`CharsetEncodeErrorKind`].
  190|       |    #[inline]
  191|     31|    pub const fn kind(self) -> CharsetEncodeErrorKind {
  192|     31|        self.kind
  193|     31|    }
  194|       |
  195|       |    /// Returns the operation index associated with this error.
  196|       |    ///
  197|       |    /// # Returns
  198|       |    ///
  199|       |    /// Returns the stored index.
  200|       |    #[inline]
  201|     29|    pub const fn index(self) -> usize {
  202|     29|        self.index
  203|     29|    }
  204|       |
  205|       |    /// Returns the raw value associated with this error.
  206|       |    ///
  207|       |    /// # Returns
  208|       |    ///
  209|       |    /// Returns `Some(value)` when the error kind carries a raw code point or
  210|       |    /// character value, or `None` for kinds without an associated value.
  211|       |    #[inline]
  212|      9|    pub const fn value(self) -> Option<u32> {
  213|      9|        self.kind.value()
  214|      9|    }
  215|       |
  216|       |    /// Offsets this error by a base unit index.
  217|       |    ///
  218|       |    /// # Parameters
  219|       |    ///
  220|       |    /// - `base`: The base index to add to the stored index.
  221|       |    ///
  222|       |    /// # Returns
  223|       |    ///
  224|       |    /// Returns a copy of this error with its index shifted by `base`.
  225|       |    ///
  226|       |    /// If the shifted index cannot be represented, it is saturated to
  227|       |    /// [`usize::MAX`].
  228|       |    #[inline]
  229|      2|    pub const fn offset_by(self, base: usize) -> Self {
  230|       |        Self {
  231|      2|            charset: self.charset,
  232|      2|            kind: self.kind,
  233|      2|            index: match self.index.checked_add(base) {
  234|      1|                Some(index) => index,
  235|      1|                None => usize::MAX,
  236|       |            },
  237|       |        }
  238|      2|    }
  239|       |}
  240|       |
  241|       |impl fmt::Display for CharsetEncodeError {
  242|       |    /// Formats this encoding error.
  243|       |    ///
  244|       |    /// # Parameters
  245|       |    ///
  246|       |    /// - `formatter`: The formatter receiving the diagnostic message.
  247|       |    ///
  248|       |    /// # Errors
  249|       |    ///
  250|       |    /// Returns any formatting error reported by `formatter`.
  251|      5|    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
  252|      5|        if let Some(value) = self.kind.value() {
                                  ^3
  253|      3|            write!(
  254|      3|                formatter,
  255|       |                "{} encoding error at index {} for value 0x{:x}: {}",
  256|       |                self.charset, self.index, value, self.kind,
  257|       |            )
  258|       |        } else {
  259|      2|            write!(
  260|      2|                formatter,
  261|       |                "{} encoding error at index {}: {}",
  262|       |                self.charset, self.index, self.kind,
  263|       |            )
  264|       |        }
  265|      5|    }
  266|       |}
  267|       |
  268|       |impl Error for CharsetEncodeError {}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/error/charset_encode_error_kind.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |use thiserror::Error;
    9|       |
   10|       |/// Classifies failures detected while encoding Unicode text into encoded units.
   11|       |#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
   12|       |#[non_exhaustive]
   13|       |pub enum CharsetEncodeErrorKind {
   14|       |    /// The supplied code point is not a valid Unicode scalar value.
   15|       |    #[error("The code point is not a valid Unicode scalar value.")]
   16|       |    InvalidCodePoint {
   17|       |        /// Raw code point value reported by the codec.
   18|       |        value: u32,
   19|       |    },
   20|       |
   21|       |    /// The requested input character index is outside the input buffer.
   22|       |    #[error("The input character index is outside the input buffer.")]
   23|       |    InvalidInputIndex {
   24|       |        /// Length of the input provided to the codec call.
   25|       |        input_len: usize,
   26|       |    },
   27|       |
   28|       |    /// The requested output unit index is outside the output buffer.
   29|       |    #[error("The output unit index is outside the output buffer.")]
   30|       |    InvalidOutputIndex {
   31|       |        /// Length of the output provided to the codec call.
   32|       |        output_len: usize,
   33|       |    },
   34|       |
   35|       |    /// The character cannot be represented by the target encoding.
   36|       |    #[error("The character cannot be represented by the target encoding.")]
   37|       |    UnmappableCharacter {
   38|       |        /// Raw character value that cannot be represented.
   39|       |        value: u32,
   40|       |    },
   41|       |
   42|       |    /// The transcode layer rejected a value without exposing its raw scalar.
   43|       |    #[error("The input value cannot be encoded by the target encoding.")]
   44|       |    UnencodableValue,
   45|       |
   46|       |    /// The transcode layer reported a failure that is not valid for encoding.
   47|       |    #[error("The transcode framework reported an unexpected encode failure.")]
   48|       |    UnexpectedTranscodeFailure,
   49|       |
   50|       |    /// The supplied output buffer is too small for the encoded character.
   51|       |    #[error("The output buffer is too small (required {required} units, available {available} units).")]
   52|       |    BufferTooSmall {
   53|       |        /// Total units required to encode the character.
   54|       |        required: usize,
   55|       |
   56|       |        /// Total units currently available for the requested output index.
   57|       |        available: usize,
   58|       |    },
   59|       |
   60|       |    /// Output length arithmetic overflowed.
   61|       |    #[error("The output length arithmetic overflowed.")]
   62|       |    OutputLengthOverflow,
   63|       |
   64|       |    /// The closed input ended before a complete value was available.
   65|       |    #[error("The input is incomplete (required {required} units, available {available} units).")]
   66|       |    IncompleteInput {
   67|       |        /// Total units required to complete the value.
   68|       |        required: usize,
   69|       |
   70|       |        /// Total units currently available from the incomplete input index.
   71|       |        available: usize,
   72|       |    },
   73|       |}
   74|       |
   75|       |impl CharsetEncodeErrorKind {
   76|       |    /// Returns the raw value associated with the error kind, if available.
   77|       |    ///
   78|       |    /// # Returns
   79|       |    ///
   80|       |    /// - `Some(value)` for [`Self::InvalidCodePoint`] and
   81|       |    ///   [`Self::UnmappableCharacter`];
   82|       |    /// - `None` for other kinds.
   83|       |    #[must_use]
   84|       |    #[inline]
   85|     22|    pub const fn value(self) -> Option<u32> {
   86|     22|        match self {
   87|      4|            Self::InvalidCodePoint { value, .. } => Some(value),
   88|      8|            Self::UnmappableCharacter { value, .. } => Some(value),
   89|       |            Self::BufferTooSmall { .. }
   90|       |            | Self::InvalidInputIndex { .. }
   91|       |            | Self::InvalidOutputIndex { .. }
   92|       |            | Self::UnencodableValue
   93|       |            | Self::UnexpectedTranscodeFailure
   94|       |            | Self::OutputLengthOverflow
   95|     10|            | Self::IncompleteInput { .. } => None,
   96|       |        }
   97|     22|    }
   98|       |
   99|       |    /// Returns the number of required output units for this kind, if available.
  100|       |    ///
  101|       |    /// # Returns
  102|       |    ///
  103|       |    /// - `Some(required)` for [`Self::BufferTooSmall`] and
  104|       |    ///   [`Self::IncompleteInput`];
  105|       |    /// - `None` for all other variants.
  106|       |    #[must_use]
  107|       |    #[inline]
  108|     10|    pub const fn required(self) -> Option<usize> {
  109|     10|        match self {
  110|      4|            Self::BufferTooSmall { required, .. } | Self::IncompleteInput { required, .. } => Some(required),
                                                 ^3                                       ^1
  111|       |            Self::InvalidInputIndex { .. }
  112|       |            | Self::InvalidOutputIndex { .. }
  113|       |            | Self::InvalidCodePoint { .. }
  114|       |            | Self::UnmappableCharacter { .. }
  115|       |            | Self::UnencodableValue
  116|       |            | Self::UnexpectedTranscodeFailure
  117|      6|            | Self::OutputLengthOverflow => None,
  118|       |        }
  119|     10|    }
  120|       |
  121|       |    /// Returns the number of currently available output units for this kind, if
  122|       |    /// available.
  123|       |    ///
  124|       |    /// # Returns
  125|       |    ///
  126|       |    /// - `Some(available)` for [`Self::BufferTooSmall`] and
  127|       |    ///   [`Self::IncompleteInput`];
  128|       |    /// - `None` for all other variants.
  129|       |    #[must_use]
  130|       |    #[inline]
  131|     10|    pub const fn available(self) -> Option<usize> {
  132|     10|        match self {
  133|      4|            Self::BufferTooSmall { available, .. } | Self::IncompleteInput { available, .. } => Some(available),
                                                 ^3                                        ^1
  134|       |            Self::InvalidInputIndex { .. }
  135|       |            | Self::InvalidOutputIndex { .. }
  136|       |            | Self::InvalidCodePoint { .. }
  137|       |            | Self::UnmappableCharacter { .. }
  138|       |            | Self::UnencodableValue
  139|       |            | Self::UnexpectedTranscodeFailure
  140|      6|            | Self::OutputLengthOverflow => None,
  141|       |        }
  142|     10|    }
  143|       |
  144|       |    /// Returns the input length when this error comes from an invalid input
  145|       |    /// index.
  146|       |    ///
  147|       |    /// # Returns
  148|       |    ///
  149|       |    /// - `Some(input_len)` for [`Self::InvalidInputIndex`];
  150|       |    /// - `None` for other variants.
  151|       |    #[must_use]
  152|       |    #[inline]
  153|      4|    pub const fn input_len(self) -> Option<usize> {
  154|      4|        match self {
  155|      1|            Self::InvalidInputIndex { input_len } => Some(input_len),
  156|       |            Self::InvalidCodePoint { .. }
  157|       |            | Self::UnmappableCharacter { .. }
  158|       |            | Self::BufferTooSmall { .. }
  159|       |            | Self::InvalidOutputIndex { .. }
  160|       |            | Self::UnencodableValue
  161|       |            | Self::UnexpectedTranscodeFailure
  162|       |            | Self::OutputLengthOverflow
  163|      3|            | Self::IncompleteInput { .. } => None,
  164|       |        }
  165|      4|    }
  166|       |
  167|       |    /// Returns the output length when this error comes from an invalid output
  168|       |    /// index.
  169|       |    ///
  170|       |    /// # Returns
  171|       |    ///
  172|       |    /// - `Some(output_len)` for [`Self::InvalidOutputIndex`];
  173|       |    /// - `None` for other variants.
  174|       |    #[must_use]
  175|       |    #[inline]
  176|      6|    pub const fn output_len(self) -> Option<usize> {
  177|      6|        match self {
  178|      3|            Self::InvalidOutputIndex { output_len } => Some(output_len),
  179|       |            Self::InvalidCodePoint { .. }
  180|       |            | Self::InvalidInputIndex { .. }
  181|       |            | Self::UnmappableCharacter { .. }
  182|       |            | Self::BufferTooSmall { .. }
  183|       |            | Self::UnencodableValue
  184|       |            | Self::UnexpectedTranscodeFailure
  185|       |            | Self::OutputLengthOverflow
  186|      3|            | Self::IncompleteInput { .. } => None,
  187|       |        }
  188|      6|    }
  189|       |}

/home/starfish/working/qubit/rust-common/rs-codec-text/src/util/label_normalize.rs:
    1|       |// =============================================================================
    2|       |//    Copyright (c) 2026 Haixing Hu.
    3|       |//
    4|       |//    SPDX-License-Identifier: Apache-2.0
    5|       |//
    6|       |//    Licensed under the Apache License, Version 2.0.
    7|       |// =============================================================================
    8|       |//! Charset label normalization helpers.
    9|       |
   10|       |/// Normalizes a charset label for this crate's permissive label matching.
   11|       |///
   12|       |/// # Parameters
   13|       |///
   14|       |/// - `label`: Label supplied by a caller.
   15|       |///
   16|       |/// # Returns
   17|       |///
   18|       |/// Returns a lowercase ASCII string after trimming ASCII whitespace and
   19|       |/// removing `-` and `_` separators. Non-ASCII characters are preserved except
   20|       |/// for ASCII case conversion.
   21|       |#[must_use]
   22|    139|pub fn normalize_label_loose(label: &str) -> String {
   23|    139|    label
   24|    139|        .trim_ascii()
   25|    139|        .chars()
   26|  2.16k|        .filter_map(|ch| match ch {
                       ^139
   27|    235|            '-' | '_' => None,
   28|  1.93k|            _ => Some(ch.to_ascii_lowercase()),
   29|  2.16k|        })
   30|    139|        .collect()
   31|    139|}
   32|       |
   33|       |/// Normalizes a charset label using WHATWG-style preprocessing.
   34|       |///
   35|       |/// # Parameters
   36|       |///
   37|       |/// - `label`: Label supplied by a caller.
   38|       |///
   39|       |/// # Returns
   40|       |///
   41|       |/// Returns a lowercase ASCII string after trimming ASCII whitespace. Unlike
   42|       |/// [`normalize_label_loose`], this function keeps punctuation and separators
   43|       |/// intact, matching the preprocessing step used before consulting the WHATWG
   44|       |/// Encoding Standard label table.
   45|       |#[must_use]
   46|     11|pub fn normalize_label_whatwg(label: &str) -> String {
   47|     11|    label.trim_ascii().to_ascii_lowercase()
   48|     11|}```

</details>
