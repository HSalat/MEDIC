# Conception

## Essential
- Define 'closed' social classes + design random draws within each class
- Social class distribution
- Check all known parameters (set to national level? -- currently set to london)
- household logic


## Non essential
- Amenities random change (maybe depend on gentrification?) -> or assume study "all other parameters remaining equal"
- Adjust life expectency for accidental death (currently included as "emmigration" for simplicity) + orphans?


# Data

- Assemble test data
- Create new Azure blob


# Coding

## Major modules
1. Create region
    - Dl & Aggregate data
    - data to model material
2. Iterate
    - step with seeding
    - step without seeding
3. Output dashboard 

## Essential
- `income_in_wneigh` and `activity_in_tneigh`

## Non essential
- Freeze period before property can be vacated again
- Add println for user's convenience
- immigration with families
- emigtration without fanilies


# Calibration

- Behaviour calibration
- Normalise accessible amenities of each type compared to the other types
- Gentrification processes