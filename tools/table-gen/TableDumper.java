import com.zvvnmod.mongolconvert.translate.shape.from.z52.FromZ52CodeMapper;
import com.zvvnmod.mongolconvert.translate.shape.to.z52.ToZ52CodeMapper;
import com.zvvnmod.mongolconvert.translate.shape.from.menk.FromMenkShapeCodeMapper;
import com.zvvnmod.mongolconvert.translate.shape.to.menk.ToMenkShapeCodeMapper;
import com.zvvnmod.mongolconvert.translate.letter.from.delehi.FromDelehiCodeMapper;
import com.zvvnmod.mongolconvert.translate.letter.to.delehi.ToDelehiCodeMapper;
import com.zvvnmod.mongolconvert.translate.letter.from.menk.FromMenkLetterCodeMapper;
import com.zvvnmod.mongolconvert.translate.letter.to.menk.ToMenkLetterCodeMapper;
import com.zvvnmod.mongolconvert.translate.word.Z52UnicodeBlock;
import com.zvvnmod.mongolconvert.translate.word.ZvvnModUnicodeBlock;
import com.zvvnmod.mongolconvert.translate.word.CodeMapper;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Generates the committed Rust lookup tables for mongol-convert by DUMPING the live Java maps
 * (the authoritative oracle), rather than parsing Java/PHP source. The maps are built by the
 * real Java static initializers (putAll 4-way expansion, buildLocateChar, concatenations, and
 * CodeMapper's duplicate-key guard), so the dump is Java-faithful by construction — zero
 * transcription or parse risk.
 *
 * Output: crates/mongol-convert/src/tables/generated/*.rs. Mappers become sorted &[(&str,&str)]
 * (binary-searchable, dependency-free); membership sets become sorted &[char].
 * Run from the repo root after `mvn compile`. Re-running regenerates deterministically.
 */
public class TableDumper {
    static final String OUT = "meco-rust/crates/mongol-convert/src/tables/generated/";

    public static void main(String[] args) throws Exception {
        Files.createDirectories(Paths.get(OUT));

        emitMapper("FROM_Z52", FromZ52CodeMapper.codeMapper, "from_z52.rs");
        emitMapper("TO_Z52", ToZ52CodeMapper.codeMapper, "to_z52.rs");
        emitMapper("FROM_MENK_SHAPE", FromMenkShapeCodeMapper.codeMapper, "from_menk_shape.rs");
        emitMapper("TO_MENK_SHAPE", ToMenkShapeCodeMapper.mapper, "to_menk_shape.rs");

        LinkedHashMap<String, Set<Character>> z52 = new LinkedHashMap<>();
        z52.put("Z52_CODES", Z52UnicodeBlock.z52Codes);
        z52.put("Z52_CODE_PUNCTUATIONS", Z52UnicodeBlock.z52CodePunctuations);
        emitCharSets(z52, "z52_block.rs");

        LinkedHashMap<String, Set<Character>> zv = new LinkedHashMap<>();
        zv.put("ZVVNMOD_CODES", ZvvnModUnicodeBlock.zvvnModCodes);
        zv.put("ZVVNMOD_TAIL_CODES", ZvvnModUnicodeBlock.zvvnModTailCodes);
        zv.put("ZVVNMOD_PUNCTUATIONS", ZvvnModUnicodeBlock.zvvnModPunctuations);
        zv.put("TO_Z52_PUNCTUATIONS", ZvvnModUnicodeBlock.toZ52Punctuations);
        emitCharSets(zv, "zvvnmod_block.rs");

        LinkedHashMap<String, Set<Character>> menk = new LinkedHashMap<>();
        menk.put("MENK_SHAPE_NOT_SUPPORT", FromMenkShapeCodeMapper.notSupportSet);
        emitCharSets(menk, "menk_shape_block.rs");

        // --- LETTER mappers (nature-split for the from-direction) ---
        LinkedHashMap<String, CodeMapper> fromDelehi = new LinkedHashMap<>();
        fromDelehi.put("FROM_DELEHI", FromDelehiCodeMapper.mapper);
        fromDelehi.put("FROM_DELEHI_CHAGH", FromDelehiCodeMapper.chaghMapper);
        fromDelehi.put("FROM_DELEHI_HUNDII", FromDelehiCodeMapper.hundiiMapper);
        fromDelehi.put("FROM_DELEHI_SAARMAG", FromDelehiCodeMapper.saarmag);
        emitMappers(fromDelehi, "from_delehi.rs");

        LinkedHashMap<String, CodeMapper> toDelehi = new LinkedHashMap<>();
        toDelehi.put("TO_DELEHI", ToDelehiCodeMapper.mapper);
        emitMappers(toDelehi, "to_delehi.rs");

        LinkedHashMap<String, CodeMapper> fromMenkLetter = new LinkedHashMap<>();
        fromMenkLetter.put("FROM_MENK_LETTER", FromMenkLetterCodeMapper.mapper);
        fromMenkLetter.put("FROM_MENK_LETTER_CHAGH", FromMenkLetterCodeMapper.chaghMapper);
        fromMenkLetter.put("FROM_MENK_LETTER_HUNDII", FromMenkLetterCodeMapper.hundiiMapper);
        fromMenkLetter.put("FROM_MENK_LETTER_SAARMAG", FromMenkLetterCodeMapper.saarmag);
        fromMenkLetter.put("FROM_MENK_LETTER_W_WITH_EHSHIG", FromMenkLetterCodeMapper.wWithEhshig);
        emitMappers(fromMenkLetter, "from_menk_letter.rs");

        LinkedHashMap<String, CodeMapper> toMenkLetter = new LinkedHashMap<>();
        toMenkLetter.put("TO_MENK_LETTER", ToMenkLetterCodeMapper.mapper);
        toMenkLetter.put("TO_MENK_LETTER_HUNDII", ToMenkLetterCodeMapper.hundiiMapper);
        toMenkLetter.put("TO_MENK_LETTER_CHAGH", ToMenkLetterCodeMapper.chaghMapper);
        emitMappers(toMenkLetter, "to_menk_letter.rs");

        System.err.println("table-gen: done");
    }

    static void emitMappers(LinkedHashMap<String, CodeMapper> maps, String file) throws IOException {
        StringBuilder b = header();
        for (Map.Entry<String, CodeMapper> m : maps.entrySet()) {
            List<Map.Entry<String, String>> entries = new ArrayList<>(m.getValue().map.entrySet());
            entries.sort(Map.Entry.comparingByKey());
            b.append("pub static ").append(m.getKey()).append(": &[(&str, &str)] = &[\n");
            for (Map.Entry<String, String> e : entries) {
                b.append("    (\"").append(rs(e.getKey())).append("\", \"").append(rs(e.getValue())).append("\"),\n");
            }
            b.append("];\n\n");
            System.err.println(file + ":" + m.getKey() + ": " + entries.size() + " entries");
        }
        Files.write(Paths.get(OUT + file), b.toString().getBytes(StandardCharsets.UTF_8));
    }

    static void emitMapper(String rustName, CodeMapper cm, String file) throws IOException {
        List<Map.Entry<String, String>> entries = new ArrayList<>(cm.map.entrySet());
        entries.sort(Map.Entry.comparingByKey());
        StringBuilder b = header();
        b.append("pub static ").append(rustName).append(": &[(&str, &str)] = &[\n");
        for (Map.Entry<String, String> e : entries) {
            b.append("    (\"").append(rs(e.getKey())).append("\", \"").append(rs(e.getValue())).append("\"),\n");
        }
        b.append("];\n");
        Files.write(Paths.get(OUT + file), b.toString().getBytes(StandardCharsets.UTF_8));
        System.err.println(file + ": " + entries.size() + " entries");
    }

    static void emitCharSets(LinkedHashMap<String, Set<Character>> sets, String file) throws IOException {
        StringBuilder b = header();
        for (Map.Entry<String, Set<Character>> s : sets.entrySet()) {
            List<Character> cs = new ArrayList<>(s.getValue());
            Collections.sort(cs);
            b.append("pub static ").append(s.getKey()).append(": &[char] = &[\n    ");
            int n = 0;
            for (char c : cs) {
                b.append(String.format("'\\u{%x}', ", (int) c));
                if (++n % 8 == 0) {
                    b.append("\n    ");
                }
            }
            b.append("\n];\n\n");
            System.err.println(file + ":" + s.getKey() + ": " + cs.size() + " chars");
        }
        Files.write(Paths.get(OUT + file), b.toString().getBytes(StandardCharsets.UTF_8));
    }

    static StringBuilder header() {
        return new StringBuilder(
            "// @generated by tools/table-gen/TableDumper.java — DO NOT EDIT.\n" +
            "// Dumped from the live Java meco maps (the authoritative oracle).\n\n");
    }

    /** Rust string-literal escape: printable ASCII (except " and \\) stays literal; else \\u{XXXX}. */
    static String rs(String s) {
        StringBuilder b = new StringBuilder(s.length() * 2);
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c >= 0x20 && c <= 0x7E && c != '"' && c != '\\') {
                b.append(c);
            } else {
                b.append("\\u{").append(Integer.toHexString(c)).append('}');
            }
        }
        return b.toString();
    }
}
