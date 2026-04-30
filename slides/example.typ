#import "@preview/touying:0.6.1": *
#import "template/ltu-theme.typ": ltu-slide, ltu-theme

#show: ltu-theme.with(
  config-info(
    subtitle: "D7020E Robust and Energy Efficient Real-Time Systems 7",
    title: "Stack Resource Policy and Scheduling Analysis",
    authors: ("Pawel Dzialo", "Prof. Per Lindgren"),
    date: "1.12.2025",
  ),
)
= Example slide
#ltu-slide[
  - This is a pretty benign slide
  - Just some bullet points
    - We can also do subbullets
]
= Pretty funky slide
#ltu-slide[
  - The two column layout and inlined raster image make this one a bit more challenging
  - For sure no match for Typst
][
#image("img/deadlock.png")
]
= Funkiest slide i could find
#ltu-slide[
  #set text(size:18pt)
  #box(width: 10fr)[
    #table(
      columns: (auto, auto, auto, auto, auto, auto, auto),
      table.header([Job], [P], [Deadline], [WCET], [Interarrival], [Response Time], [$B_i$]),
      stroke: white,
      [$J_1$], [$1$], [$56"ms"$], [$10"ms"$], [$100"ms"$], [$20"ms"$], [$0"ms"$],
      [$J_2$], [$2$], [$20"ms"$], [$6"ms"$], [$70"ms"$], [$10"ms"$], [$2"ms"$],
      [$J_3$], [$3$], [$10"ms"$], [$2"ms"$], [$12"ms"$], [$2"ms"$], [$0"ms"$],
    )
  ]
  #box(width: 5fr)[
    #table(
      columns: (auto, auto, auto, auto),
      table.header([CS], [Resource], [WCET], [In Job]),
      stroke: white,
      [$Z_(1,1)$], [$S_1$], [$2"ms"$], [$J_1$],
      [$Z_(2,1)$], [$S_1$], [$4"ms"$], [$J_2$],
    )
  ]
  - $R_1^((3)) = B_1 + C_1 + C_2 * ceil(R_1^((3))/A_2) + C_3*ceil(R_1^((3))/A_3) =#linebreak()
    = 10"ms" + 6"ms" * ceil((20"ms")/(70"ms")) + 2"ms" *ceil((20"ms")/(12"ms")) = 10"ms" + 6"ms"*1 + 2"ms"*2 = 20"ms"$
  - Indeed, we converge around $R_1 = 20"ms"$
  #box(width: 1fr)[
    #set align(center)
    #image("img/scheduling8.png", height: 39%)
  ]
]
