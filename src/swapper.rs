extern crate clap;
extern crate rustbox;

mod alphabets;
mod colors;
mod state;
mod view;

use std::process::Command;

fn exec_command(args: Vec<&str>) -> std::process::Output {
  return Command::new(args[0])
    .args(&args[1..])
    .output()
    .expect("Couldn't run it");
}

fn active_pane() -> String {
  let active_command = vec!["tmux", "list-panes", "-F", "#{pane_id}:#{?pane_active,active,nope}"];
  let execution = exec_command(active_command);
  let output = String::from_utf8_lossy(&execution.stdout);
  let lines: Vec<&str> = output.split("\n").collect();
  let chunks: Vec<Vec<&str>> = lines
    .into_iter()
    .map(|line| line.split(":").collect())
    .collect();

  let chunk = chunks.iter().find(|&chunks| {
    *chunks.iter().nth(1).unwrap() == "active"
  }).expect("Unable to find active pane");

  chunk.iter().nth(0).unwrap().to_string()
}

fn thumbs_pane() -> String {
  let string_params = vec![
    "position",
    "fg-color",
    "bg-color",
    "hint-bg-color",
    "hint-fg-color",
    "select-fg-color",
    "command",
    "upcase-command"
  ];

  let boolean_params = vec![
    "reverse",
    "unique",
    "contrast"
  ];

  let _multi_params = vec![
    "regexp"
  ];

  let mut thumbs_command = vec!["tmux", "new-window", "-P", "-d", "-n", "[thumbs]"];

  let foo1: Vec<[String; 2]> = string_params.iter().map(|param| {
    let thumbs_param = format!("@thumbs-{}", param);
    let command = vec![
      "tmux",
      "show",
      "-vg",
      &thumbs_param
    ];

    let execution = exec_command(command);
    let output = String::from_utf8_lossy(&execution.stdout);
    let value = output.trim_end();

    if value.is_empty() {
      None
    } else {
      Some([format!("--{}", param).to_string(), value.to_string()])
    }
  }).flat_map(|e| e).collect();

  let foo2: Vec<String> = foo1.iter().flatten().cloned().collect::<Vec<_>>();
  let foo3: Vec<&str> = foo2.iter().map(AsRef::as_ref).collect::<Vec<_>>();

  let foo: Vec<String> = boolean_params.iter().map(|param| {
    let thumbs_param = format!("@thumbs-{}", param);
    let command = vec![
      "tmux",
      "show",
      "-vg",
      &thumbs_param
    ];

    let execution = exec_command(command);
    let output = String::from_utf8_lossy(&execution.stdout);
    let value = output.trim_end();

    if value.is_empty() {
      None
    } else {
      Some(format!("--{}", param).to_string())
    }
  }).flat_map(|e| e).collect();
  let foo4: Vec<&str> = foo.iter().map(AsRef::as_ref).collect::<Vec<_>>();

  thumbs_command.push("/target/release/tmux-thumbs");
  thumbs_command.extend(foo3);
  thumbs_command.extend(foo4);

  let execution = exec_command(thumbs_command);

  String::from_utf8_lossy(&execution.stdout).trim_end().to_string()
}

fn main() {
  // tmux list-panes -F "#{pane_id}:#{?pane_active,active,nope}" | grep active | cut -d: -f1
  // tmux new-window -P -d -n "[thumbs]" ${CURRENT_DIR}${TARGET_RELEASE}tmux-thumbs "${PARAMS[@]}" "--tmux-pane=${CURRENT_PANE_ID}"
  // tmux list-panes -a | grep ${NEW_ID} | grep --color=never -o '%[0-9]\+'
  // tmux swap-pane -d -s ${CURRENT_PANE_ID} -t ${NEW_PANE_ID}


  let active_pane_id = active_pane();
  let thumbs_pane_id = thumbs_pane();

  println!("FCS: {:?}", active_pane_id);
  println!("FCS2: {:?}", thumbs_pane_id);

  // let current_pane_id = "12";
  // let new_pane_id = "14";

  // // Retrieve current active pane in tmux session
  // let active_command = vec!["tmux", "list-panes", "-F", "#{pane_id}:#{?pane_active,active,nope}"];

  // let execution = exec_command(active_command);
  //   let output = String::from_utf8_lossy(&execution.stdout);

  // // Swap old pane with the new pane
  // let swap_command = vec!["tmux", "swap-pane", "-d", "-s", current_pane_id, "-t", new_pane_id];

  // let mut capture_command = vec!["tmux", "new-window", "-P", "-d", "-n", "[thumbs]"];

  // string_params.iter().for_each(|param| {
  //   let thumbs_param = format!("@thumbs-{}", param);
  //   let command = vec![
  //     "tmux",
  //     "show",
  //     "-vg",
  //     &thumbs_param
  //   ];

  //   let execution = exec_command(command);
  //   let output = String::from_utf8_lossy(&execution.stdout);

  //   capture_command.extend(vec![param, output].iter().cloned());
  // });

  // println!("FCS: {:?}", capture_command);

  // if let Some(pane) = args.value_of("tmux_pane") {
  //   capture_command.extend(vec!["-t", pane].iter().cloned());
  // }

  // let execution = exec_command(capture_command);
  // let output = String::from_utf8_lossy(&execution.stdout);
  // let lines = output.split("\n").collect::<Vec<&str>>();

  // if let Some(pane) = args.value_of("tmux_pane") {
  //   exec_command(vec!["tmux", "swap-pane", "-t", pane]);
  // };

  // if paste {
  //   exec_command(vec![
  //     "bash",
  //     "-c",
  //     str::replace(upcase_command, "{}", text.as_str()).as_str(),
  //   ]);
  // }
}
