<script setup lang="ts">
import Terminal from "@/pages/Terminal.vue";
import {ref} from "vue";

interface Tree {
  label: string
  children?: Tree[]
}

const handleNodeClick = (data: Tree) => {
  console.log(data)
}

const data: Tree[] = [
  {
    label: 'Level one 1',
    children: [
      {
        label: 'Level two 1-1',
        children: [
          {
            label: 'Level three 1-1-1',
          },
        ],
      },
    ],
  },
  {
    label: 'Level one 2',
    children: [
      {
        label: 'Level two 2-1',
        children: [
          {
            label: 'Level three 2-1-1',
          },
        ],
      },
      {
        label: 'Level two 2-2',
        children: [
          {
            label: 'Level three 2-2-1',
          },
        ],
      },
    ],
  },
  {
    label: 'Level one 3',
    children: [
      {
        label: 'Level two 3-1',
        children: [
          {
            label: 'Level three 3-1-1',
          },
        ],
      },
      {
        label: 'Level two 3-2',
        children: [
          {
            label: 'Level three 3-2-1',
          },
        ],
      },
    ],
  },
]

const defaultProps = {
  children: 'children',
  label: 'label',
}






const terminal = ref(null);
const logLines = ref([
  "Line 1: Initialization complete.",
  "Line 2: Starting process...",
  "Line 3: Process running...",
  "Line 4: Process finished.",
]);
let currentLineIndex = 0;

const addLogLine = () => {
  if (currentLineIndex < 100) {
    terminal.value.addLine(logLines.value[0]);
    currentLineIndex++;
  }
};
</script>

<template>
<!--    <button @click="addLogLine">Add Log Line</button>-->
  <Terminal ref="terminal" />

<!--  <el-tree-->
<!--      style="max-width: 600px"-->
<!--      :data="data"-->
<!--      :props="defaultProps"-->
<!--      @node-click="handleNodeClick"-->
<!--  />-->
</template>

<style scoped>

</style>