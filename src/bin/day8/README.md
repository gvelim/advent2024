## Advent of Code 2024 - Day 8: Antenna Problem

### Problem Statement
The challenge involves a city grid where certain locations are marked with antennas. Each antenna can interact with other antennas to form antinodes at specific harmonic intervals. The goal is to determine the number of unique locations within the city grid that contain an antinode, both for a single harmonic and for a range of harmonics.

### Intuition and Approach
The problem can be broken down into the following steps:
1. **Parsing the Input**: Read the city grid and identify the locations of the antennas.
2. **Calculating Antinodes**: For each pair of antennas, calculate the potential antinode locations based on the given harmonic intervals.
3. **Filtering Valid Locations**: Ensure that the calculated antinode locations are within the bounds of the city grid.
4. **Counting Unique Locations**: Determine the number of unique locations that contain an antinode.

### Detailed Steps and Code Snippets

1. **Parsing the Input**
   The input is parsed into a `City` structure, which contains a grid (`Field<char>`) and a hashmap of antennas grouped by their identifiers.

   ```rust
   impl FromStr for City {
       type Err = ();

       fn from_str(s: &str) -> Result<Self, Self::Err> {
           let city = s.parse::<Field<char>>()?;
           let antennas: HashMap<char,Vec<Antenna>> = city.iter()
               .enumerate()
               .filter(|&(_,c)| c.ne(&'.'))
               .fold(HashMap::new(), |mut map, (i, &c)| {
                   let loc = city.index_to_cartesian(i);
                   map.entry(c)
                       .and_modify(|antennas| antennas.push(Antenna(loc)))
                       .or_insert(vec![Antenna(loc)]);
                   map
               });
           Ok(City { city, antennas })
       }
   }
   ```

2. **Calculating Antinodes**
   The `Antenna` struct has methods to calculate antinode pairs for given harmonics. The `antinode_pair` method computes the relative positions of antinodes based on the distance between two antennas and the harmonic factor.

  ```rust
  impl Antenna {
      pub fn antinode_pair(
          &self,
          rhs: Antenna,
          harmonics: usize
      ) -> [Option<Location>;2]
      {
          let (dxu,dyu) = self.0.distance(&rhs.0);
          let (dx,dy) = ((harmonics * dxu) as isize, (harmonics * dyu) as isize);
          match (self.0.0 >= rhs.0.0, self.0.1 >= rhs.0.1) {
              (true, true) => [rhs.0.move_relative((-dx,-dy)), self.0.move_relative((dx,dy))],
              (true, false) => [rhs.0.move_relative((-dx,dy)), self.0.move_relative((dx,-dy))],
              (false, true) => [rhs.0.move_relative((dx,-dy)), self.0.move_relative((-dx,dy))],
              (false, false) => [rhs.0.move_relative((dx,dy)), self.0.move_relative((-dx,-dy))],
          }
      }

      pub fn antinodes(
          &self, rhs: Antenna,
          harmonics: RangeInclusive<usize>
      ) -> impl Iterator<Item =[Option<Location>;2]>
      {
          harmonics.map(move |harmonics| self.antinode_pair(rhs, harmonics))
      }
  }
  ```

3. **Filtering Valid Locations**
   The `City` struct's `antinodes` method filters out antinode locations that are outside the bounds of the city grid.

   ```rust
   impl City {
       pub fn antinodes(&self, harmonics:RangeInclusive<usize>) -> impl Iterator<Item = Location> {
           self.antennas
               .values()
               .flat_map(move |antennas| antennas
                   .iter()
                   .tuple_combinations()
                   .flat_map({
                       let h = harmonics.clone();
                       move |(a, b)| a
                           .antinodes(*b, h.clone())
                           .take_while(|&antinodes| {
                               match (antinodes[0], antinodes[1]) {
                                   (_, Some(l)) if self.city.get(l).is_some() => true,
                                   (Some(l), _) if self.city.get(l).is_some() => true,
                                   _ => false
                               }
                           })
                   })
               )
               .flat_map(|antinodes| antinodes.into_iter())
               .filter_map(|location|
                   location.filter(|&location| self.city.get(location).is_some())
               )
       }
   }
   ```

4. **Counting Unique Locations**
   The main function reads the input, parses it into a `City` object, and then counts the unique antinode locations for the specified harmonic ranges.

   ```rust
   fn main() {
       let input = std::fs::read_to_string("src/bin/day8/input.txt").unwrap();
       let city = input.parse::<City>().expect("Failed to parse City");

       let t = Instant::now();
       let count = city.antinodes(1..=1).unique().count();
       println!("Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}",count, t.elapsed());
       assert_eq!(247,count);

       let t = Instant::now();
       let count = city.antinodes(0..=100).unique().count();
       println!("Part 2: {:?} unique locations contain an antinode given the effects of resonant harmonics - {:?}",count, t.elapsed());
       assert_eq!(861,count);
   }
   ```
