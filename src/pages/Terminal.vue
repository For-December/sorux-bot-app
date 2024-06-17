<template>
  <el-scrollbar>
    <div class="bg-[black]" ref="terminalContainer"></div>
  </el-scrollbar>

</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

export default defineComponent({
  name: 'Terminal',
  setup() {
    const terminalContainer = ref<HTMLElement | null>(null);
    const terminal = ref<Terminal | null>(null);

    onMounted(() => {
      if (terminalContainer.value) {
        const term = new Terminal();
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(terminalContainer.value);
        fitAddon.fit();

        terminal.value = term;

        // Example: Write initial content to the terminal
        term.write('Welcome to the Vue.js terminal!\r\n');
      }
    });

    // Function to add a new line of text to the terminal
    const addLine = (line: string) => {
      terminal.value?.writeln(line);
    };

    return {
      terminalContainer,
      addLine,
    };
  },
});
</script>

<style scoped>

</style>
