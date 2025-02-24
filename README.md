# Bevy Boids

A boid simulator in bevy.

## Dev Notes

- referec
- each boid has a detection component
  - distance
  - angle (from local FORWARD)
- each property has shared values
  - effect multiplier
- Separation
  - combined position of local flockmates
  - inverse magnitude
- Alignment
  - average heading
  - direct magnitude
- Cohesion
  - average position
  - direct magnitude

## References

- [The Coding Train](https://www.youtube.com/watch?v=mhjuuHl6qHM)
- [Boids by Craig Raynolds](https://red3d.com/cwr/boids/)
- [Natsu-Anon](https://github.com/natsu-anon/BoidsDemo)

## Issues

- not performant (should use shader code and/or quad-trees)