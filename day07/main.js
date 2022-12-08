'use strict'

const fs = require("fs");

function Node(name) {   
   this.name = name
   this.size = 0
   this.children = []
   this.parent = ''
}

function get_size(lookup, name) {  
   let to_visit = [name]
   let visited = []
   let val = 0

   while (to_visit.length) {
      let n = to_visit.pop()
   
      val += lookup[n].size

      for (name of lookup[n].children) {
         
         if (!visited.includes(name)) {
            to_visit.push(name)
         }
      }
      visited.push(n)

   }

   return val
}

function solve (commands) {
   let nodes = {}
   nodes["root"] = new Node("root")

   let cur_node = nodes["root"]

   for (let c of commands) {
      let command = c.split(" ")   
      
      if (command[0] === '$') {
         if (command[2] === '..' && cur_node.name !== "/" ) {
            cur_node = nodes[cur_node.parent]
         } else if (command[1] === 'cd') {   
            let n = cur_node.children.find(n => n === `${cur_node.name}/${command[2]}`)            
            cur_node = nodes[n]              
         }
      } else {
         // ls
         if (command[0] === 'dir') {
            let node_name = `${cur_node.name}/${command[1]}`
            nodes[node_name] = new Node(node_name)
            nodes[node_name].parent = cur_node.name
          
            cur_node.children.push(node_name)
           
         } else  {
            cur_node.size += parseInt(command[0], 10)
         }
      }
   }   

   let tot = 0

   const keys = Object.keys(nodes)

   const max_size =  70000000
   const target_size = 30000000
   const total_size = get_size(nodes, "root")
   const used_space = max_size- total_size;
   let best_fit = used_space
  
   console.log(`total size: ${total_size}, used: ${used_space} remaining: ${total_size - used_space}`)

   for (let key of keys) {
      let num = get_size(nodes, key)
      //console.log(`num: ${num}`)
      if (max_size - (total_size - num) >= target_size && num <= best_fit ) {
         best_fit = num
        
      }
      // part 1 logic
      // if (num <= 100000) {
      //    tot += num
      // }


   }

   //console.log(nodes)

   return best_fit
}

try {
   const data =  fs.readFileSync("./puzzle.txt", { encoding: "utf8", flag: "r" });
   const commands =  data.split(/r?\n/)

   commands.shift()

   //console.log("commands ", commands)

   let ans = solve(commands)
   console.log(`Answer: ${ans}`)
   
   
} catch (e) {
   console.log("ERROR", e)
}
    
    