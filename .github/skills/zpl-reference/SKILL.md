---
name: zpl-reference
description: "Look up ZPL command specifications from the official Zebra Programming Guide. Use when: implementing ZPL commands, debugging ZPL parsing, verifying parameter defaults, checking command interactions, fixing rendering that doesn't match expected ZPL behavior, understanding barcode or font or field or graphic command semantics."
argument-hint: "ZPL command name or rendering question (e.g. '^BC parameters', 'field block word wrap')"
---

# ZPL Official Reference Lookup

## Purpose

Fetch and interpret ZPL command specifications from the official Zebra Programming Guide to resolve ambiguities in labelize's ZPL parser and renderer.

## When to Use

- A rendered label doesn't match expected output
- Implementing or fixing a ZPL command parser
- Unsure about parameter defaults, ranges, or optional values
- Command interaction behavior is unclear (e.g., `^FO` vs `^FT`, `^BY` defaults)
- Adding support for a new ZPL command

## Official Documentation

**Base URL:** `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zpl-commands.html`

### Command Page URL Pattern

Each ZPL command has a dedicated page. The URL follows this pattern:

```
https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zpl-commands/r-zpl-<slug>.html
```

### Common Command Slugs

| Command | Slug | Description |
|---------|------|-------------|
| `^A` | `a` | Scalable/bitmapped font |
| `^A@` | `a1` | Use font name to call font |
| `^B0` | `b0` | Aztec barcode |
| `^B2` | `b2` | Interleaved 2-of-5 barcode |
| `^B3` | `b3` | Code 39 barcode |
| `^B7` | `b7` | PDF417 barcode |
| `^BC` | `bc` | Code 128 barcode |
| `^BD` | `bd` | UPS MaxiCode barcode |
| `^BE` | `be` | EAN-13 barcode |
| `^BQ` | `bq` | QR Code barcode |
| `^BX` | `bx` | DataMatrix barcode |
| `^BY` | `by` | Barcode field default |
| `^CF` | `cf` | Change alphanumeric default font |
| `^CI` | `ci` | Change international font |
| `^DF` | `df` | Download format |
| `^FD` | `fd` | Field data |
| `^FB` | `fb` | Field block |
| `^FH` | `fh` | Field hexadecimal indicator |
| `^FN` | `fn` | Field number |
| `^FO` | `fo` | Field origin |
| `^FR` | `fr` | Field reverse print |
| `^FS` | `fs` | Field separator |
| `^FT` | `ft` | Field typeset |
| `^FV` | `fv` | Field variable |
| `^FW` | `fw` | Field orientation |
| `^GB` | `gb` | Graphic box |
| `^GC` | `gc` | Graphic circle |
| `^GD` | `gd` | Graphic diagonal line |
| `^GF` | `gf` | Graphic field |
| `^GS` | `gs` | Graphic symbol |
| `^IL` | `il` | Image load |
| `^LH` | `lh` | Label home |
| `^LL` | `ll` | Label length |
| `^LR` | `lr` | Label reverse print |
| `^PO` | `po` | Print orientation |
| `^PW` | `pw` | Print width |
| `^XA` | `xa` | Start format |
| `^XF` | `xf` | Recall format |
| `^XG` | `xg` | Recall graphic |
| `^XZ` | `xz` | End format |
| `~DG` | `dg` | Download graphics |

### Supplementary Pages

| Topic | URL |
|-------|-----|
| Field Interactions | `https://docs.zebra.com/us/en/printers/software/zpl-pg/r-zpl-interactions-field-interactions.html` |
| Fonts and Barcodes | `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-font-barcodes-fonts-andbar-codes.html` |
| Character Encoding | `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zebra-code-zebra-code-pages.html` |
| ZB64 Encoding | `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zb64-encoding-zb64-encoding-compression.html` |

## Procedure

1. **Identify the command** — Extract the ZPL command prefix (e.g., `^BC`, `^FO`, `^GB`) from the user's question or from the parser code.

2. **Fetch the official doc** — Use the `#tool:fetch_webpage` tool to load the command's page:
   ```
   https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zpl-commands/r-zpl-<slug>.html
   ```

3. **Extract key details:**
   - Parameter list with positions, types, defaults, and valid ranges
   - Interaction notes (e.g., "this command is affected by `^BY`")
   - Default behavior when parameters are omitted

4. **Compare with labelize implementation:**
   - Search the parser for how the command is parsed: `grep_search` for the command prefix in `src/parsers/`
   - Check the element struct in `src/elements/`
   - Check the renderer logic in `src/drawers/renderer.rs`

5. **Report discrepancies** between the official spec and labelize's implementation.

## Example

User asks: "Why is my Code 128 barcode too wide?"

1. Fetch `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zpl-commands/r-zpl-bc.html`
2. Check `^BC` parameters: orientation, height, print interpretation line, above/below, UCC check digit, mode
3. Fetch `https://docs.zebra.com/us/en/printers/software/zpl-pg/c-zpl-zpl-commands/r-zpl-by.html`
4. Check `^BY` defaults: module width (default 2), wide-to-narrow ratio (default 3.0), bar height (default 10)
5. Search labelize parser for `^BY` handling — verify defaults match
6. Report findings
