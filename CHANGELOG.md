# 0.2.1
## Minor Changes
- Changed LICENSE from MIT to MIT-0
- Updated the README
- Various changes to the documentation
  - fixed some overlooked formatting mistakes
  - structs that derive traits now outline what traits they derive in their descriptions
  - re-wrote some descriptions to be more accurate about what the function is actually doing
- Removed u8_div_overflow_min_clamp
  - useless function I added because it was in the rust documentation lol
- Changed how Matrix4x4 implements Display
## Bug fixes
- degrees_overflow was taking the abs of input value instead of adding 360.0 when the input was negative
- Cargo.toml categories section now uses correct category slugs
## Notes
- I need to drink more coffee if I'm gonna make a library public

# 0.2.0
## Bug fixes
- Matrix indeces for translation values were incorrect. They *should* be the first 3 cells in the last column, they were instead the first 3 cells in the last row.

# 0.1.0
- First Version