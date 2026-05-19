#!/usr/bin/env python3
from __future__ import annotations

import argparse
import glob
import json
import os
import re
import statistics
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlparse


FAKE_RESULT_PATTERNS = [
    re.compile(r"\bresult\s*(?:\{|//)"),
    re.compile(r"结果\s*\{"),
    re.compile(r"data\s*↦\s*[「\"].*?[」\"]\s*🏷\s*[\w\"-]+", re.DOTALL),
    re.compile(r"status\s*:\s*(?:success|failure).*?exit_code\s*:", re.DOTALL),
]


def has_fake_result(text: str) -> bool:
    return any(pattern.search(text) for pattern in FAKE_RESULT_PATTERNS)


def rollout_paths(run_dir: Path) -> list[str]:
    rollouts_dir = run_dir / "run_default" / "rollouts"
    if not rollouts_dir.exists():
        rollouts_dir = run_dir / "rollouts"
    return sorted(
        glob.glob(str(rollouts_dir / "step_*" / "train_rollouts.jsonl")),
        key=lambda path: int(os.path.basename(os.path.dirname(path)).split("_")[1]),
    )


def summarize(path: str) -> dict:
    rewards: list[float] = []
    fake = tools = no_call = zeros = pos = posfake = 0
    lengths: list[int] = []

    with open(path, encoding="utf-8") as handle:
        for raw in handle:
            row = json.loads(raw)
            reward = float(row.get("reward", 0.0))
            rewards.append(reward)
            pos += reward > 0
            assistant = "\n".join(
                message.get("content", "")
                for message in row.get("completion", [])
                if isinstance(message, dict) and message.get("role") == "assistant"
            )
            fake_result = has_fake_result(assistant)
            fake += fake_result
            posfake += fake_result and reward > 0
            no_call += "act {" not in assistant
            tools += sum(
                1
                for message in row.get("completion", [])
                if isinstance(message, dict) and message.get("role") == "tool"
            )
            zeros += bool(row.get("filters", {}).get("zero_advantage"))
            lengths.append(len(assistant))

    step = int(os.path.basename(os.path.dirname(path)).split("_")[1])
    return {
        "step": step,
        "avg": statistics.mean(rewards),
        "min": min(rewards),
        "max": max(rewards),
        "pos": pos,
        "posfake": posfake,
        "no_call": no_call,
        "fake": fake,
        "tools": tools,
        "zero": zeros,
        "len": round(statistics.mean(lengths)),
    }


def data_payload(run_dir: Path) -> dict:
    rows = [summarize(path) for path in rollout_paths(run_dir)]
    latest = rows[-10:]
    summary = {}
    if latest:
        summary = {
            "avg_reward": statistics.mean(row["avg"] for row in latest),
            "avg_pos": statistics.mean(row["pos"] for row in latest),
            "avg_no_call": statistics.mean(row["no_call"] for row in latest),
            "avg_fake": statistics.mean(row["fake"] for row in latest),
            "avg_tools": statistics.mean(row["tools"] for row in latest),
            "avg_len": statistics.mean(row["len"] for row in latest),
        }
    return {"run_dir": str(run_dir), "rows": rows, "summary": summary}


HTML = """<!doctype html>
<meta charset="utf-8">
<title>RL Rollout Viewer</title>
<style>
body{margin:0;font:14px system-ui,Arial;background:#0f1115;color:#e8e8e8}
main{padding:18px;max-width:1280px;margin:auto}
h1{font-size:18px;margin:0 0 10px}
.meta{color:#aeb4c2;margin-bottom:14px}
.grid{display:grid;grid-template-columns:repeat(6,1fr);gap:8px;margin-bottom:14px}
.stat{background:#181c24;border:1px solid #2a3040;border-radius:6px;padding:10px}
.stat b{display:block;font-size:18px;color:#fff}
canvas{width:100%;height:520px;background:#151922;border:1px solid #2a3040;border-radius:6px}
table{width:100%;border-collapse:collapse;margin-top:14px;background:#151922}
td,th{border-bottom:1px solid #2a3040;padding:6px;text-align:right}
th:first-child,td:first-child{text-align:left}
</style>
<main>
<h1>RL Rollout Viewer</h1>
<div class="meta" id="meta"></div>
<div class="grid" id="stats"></div>
<canvas id="chart" width="1200" height="520"></canvas>
<table id="table"></table>
</main>
<script>
const metrics=[
  ["avg","#7dd3fc"],["pos","#86efac"],["fake","#fca5a5"],
  ["no_call","#fcd34d"],["tools","#c4b5fd"],["len","#fdba74"]
];
function fmt(x){return Number.isFinite(x)?x.toFixed(2):""}
function yscale(v,key){
  if(key==="avg") return v/3.5;
  if(key==="len") return v/2200;
  return v/48;
}
function draw(rows){
  const c=document.getElementById("chart"),ctx=c.getContext("2d");
  ctx.clearRect(0,0,c.width,c.height);
  const pad=46,w=c.width-pad*2,h=c.height-pad*2;
  ctx.strokeStyle="#303747";ctx.lineWidth=1;
  for(let i=0;i<=4;i++){let y=pad+h*i/4;ctx.beginPath();ctx.moveTo(pad,y);ctx.lineTo(pad+w,y);ctx.stroke();}
  ctx.fillStyle="#aeb4c2";ctx.fillText("higher is better for avg/pos/tools; lower is better for fake/no_call/len",pad,20);
  metrics.forEach(([key,color],mi)=>{
    ctx.strokeStyle=color;ctx.lineWidth=2;ctx.beginPath();
    rows.forEach((r,i)=>{
      const x=pad+(rows.length<=1?0:i*w/(rows.length-1));
      const y=pad+h-(Math.max(0,Math.min(1,yscale(r[key],key)))*h);
      if(i===0)ctx.moveTo(x,y);else ctx.lineTo(x,y);
    });
    ctx.stroke();ctx.fillStyle=color;ctx.fillText(key,pad+mi*95,c.height-14);
  });
}
async function refresh(){
  const res=await fetch("/data"); const data=await res.json(); const rows=data.rows;
  const last=rows[rows.length-1]||{}, s=data.summary||{};
  document.getElementById("meta").textContent=`${data.run_dir} | latest step ${last.step ?? "none"} | refresh 5s`;
  document.getElementById("stats").innerHTML=[
    ["latest reward",last.avg],["latest pos",last.pos],["latest fake",last.fake],
    ["latest no_call",last.no_call],["latest tools",last.tools],["latest len",last.len]
  ].map(([k,v])=>`<div class=stat>${k}<b>${fmt(v)}</b></div>`).join("");
  draw(rows);
  document.getElementById("table").innerHTML="<tr>"+["step","avg","min","max","pos","posfake","no_call","fake","tools","zero","len"].map(x=>`<th>${x}</th>`).join("")+"</tr>"+
    rows.slice(-30).reverse().map(r=>"<tr>"+["step","avg","min","max","pos","posfake","no_call","fake","tools","zero","len"].map(k=>`<td>${fmt(r[k])}</td>`).join("")+"</tr>").join("");
}
refresh(); setInterval(refresh,5000);
</script>
"""


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("run_dir", type=Path)
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8090)
    parser.add_argument("--terminal", action="store_true")
    parser.add_argument("--interval", type=float, default=5.0)
    args = parser.parse_args()

    if args.terminal:
        while True:
            payload = data_payload(args.run_dir)
            rows = payload["rows"][-20:]
            print("\033[2J\033[H", end="")
            print(f"RL rollout viewer: {payload['run_dir']}")
            print("step avg min max pos posfake no_call fake tools zero len")
            for row in rows:
                print(
                    row["step"],
                    round(row["avg"], 4),
                    round(row["min"], 2),
                    round(row["max"], 2),
                    row["pos"],
                    row["posfake"],
                    row["no_call"],
                    row["fake"],
                    row["tools"],
                    row["zero"],
                    row["len"],
                )
            if payload["summary"]:
                print("latest10", {k: round(v, 3) for k, v in payload["summary"].items()})
            time.sleep(args.interval)

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            path = urlparse(self.path).path
            if path == "/data":
                body = json.dumps(data_payload(args.run_dir)).encode()
                self.send_response(200)
                self.send_header("content-type", "application/json")
                self.end_headers()
                self.wfile.write(body)
                return
            self.send_response(200)
            self.send_header("content-type", "text/html; charset=utf-8")
            self.end_headers()
            self.wfile.write(HTML.encode())

        def log_message(self, *_args) -> None:
            return

    server = ThreadingHTTPServer((args.host, args.port), Handler)
    print(f"serving http://{args.host}:{args.port}")
    server.serve_forever()


if __name__ == "__main__":
    main()
