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
#speaker-note[
  Here are some notes for our noble speaker
]
]
= Pretty funky slide
#ltu-slide[
  - The two column layout and inlined raster image make this one a bit more challenging
  - For sure no match for Typst
#speaker-note[
Example speaker notes.
 
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce bibendum ante gravida, tempus neque at, euismod nibh. Etiam nec ultrices purus. Vestibulum vulputate consequat enim in sodales. Pellentesque non malesuada enim. Lorem ipsum dolor sit amet, consectetur adipiscing elit. In hac habitasse platea dictumst. Vestibulum dictum justo eget dolor cursus efficitur. Nulla sodales elit a condimentum convallis. Nam a magna non massa ultrices porta. In ac mauris ut est pulvinar pretium. Donec ac imperdiet massa. Cras in elit elementum, congue nibh ultrices, vestibulum purus. Mauris auctor, augue vel maximus vestibulum, sem odio semper enim, in bibendum lacus turpis sed magna. Proin consequat vestibulum elit, tincidunt gravida arcu posuere id. Sed tristique lorem ac nisl mattis, nec pulvinar elit eleifend. Fusce quam tellus, scelerisque eu accumsan nec, posuere ultricies urna.

Donec elit ante, venenatis ac tempor et, hendrerit id tellus. Duis posuere commodo erat, non rutrum elit auctor ac. Cras congue mi ac quam aliquet, nec sagittis justo dapibus. In quis finibus dui. Cras eu imperdiet velit, ac fermentum est. Sed libero nunc, eleifend eget rutrum nec, bibendum et nunc. Maecenas et turpis convallis, varius sapien id, gravida massa. Nunc venenatis sit amet est aliquet consectetur. Nunc quis magna et tellus consectetur scelerisque. Duis fermentum enim non laoreet finibus. Nulla pulvinar luctus elit, sed accumsan nibh bibendum at. Nulla tellus quam, luctus tempus venenatis mollis, scelerisque in felis.

Aliquam sodales magna eu diam dignissim commodo. Duis blandit metus eget nunc facilisis consequat. Ut sit amet ornare nisl. Fusce viverra auctor magna nec facilisis. Nullam viverra ac metus id porta. Etiam ligula nisi, porttitor sed interdum sollicitudin, viverra id risus. Etiam vel ipsum vel lectus semper euismod. Quisque sed condimentum lorem, et condimentum mauris. Mauris dictum, tortor non interdum bibendum, velit arcu rutrum arcu, quis mattis arcu enim sit amet eros. Nullam commodo pellentesque dictum. Nullam in dignissim sapien, sit amet facilisis diam. Mauris a mi luctus, feugiat nisi sed, euismod leo. Proin efficitur quam sed elit iaculis scelerisque ut sit amet augue.

Donec eu metus faucibus enim auctor facilisis id lacinia diam. Proin ultricies velit sed dui feugiat lacinia. Proin vestibulum scelerisque sem, at laoreet tortor venenatis quis. Etiam rutrum, ligula sit amet facilisis mattis, ligula quam cursus est, in tincidunt elit dolor sit amet nisl. Quisque porta eros non cursus imperdiet. Vestibulum quis sem sit amet turpis efficitur rutrum in non enim. Praesent elementum, risus in dapibus congue, nisl arcu laoreet nisi, et auctor urna lectus ut ante. Nam sit amet sem vel dolor aliquam sodales sed at nulla. Donec sodales orci fringilla ligula faucibus, non pharetra ipsum faucibus. Sed ullamcorper malesuada velit, a rhoncus sem gravida in.

Sed pretium venenatis convallis. Maecenas vel cursus est. Etiam id pulvinar justo, ac laoreet dolor. Mauris a massa et lacus imperdiet commodo sed sed mauris. Maecenas vulputate elit placerat tortor fringilla, id ultrices est pulvinar. Nunc et nisl vitae eros feugiat aliquet. Aliquam eleifend suscipit elit cursus venenatis. Maecenas urna nunc, consequat quis interdum nec, scelerisque eget mi. ]
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
  #speaker-note[
    The speaker may speak for ages, the view should be scrollable whenever the text would otherwise overflow the view. Here is a long paragraph of text to demonstrate this functionality. To repeat, here we are just trying to overflow the notes text box. I am running of of things to write here is some Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eui fugiat







      Here is some more text with a bunch of linebreaks in between, how does the parser see this?

    ]
    
]

= Example slide without notes
#ltu-slide[
  - The tool should now support slides without notes
]

= Back to noting
#ltu-slide[
  - This slide should have a note again
  #speaker-note[
    Indeed, here are the notes for the final slide.
  ]
]
