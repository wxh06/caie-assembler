<script setup lang="ts">
import { ref } from "vue";
import { Assembler, Location, type Step } from "@caie-assembler/pkg";
import AssemblyEditor, {
  type Instructions,
} from "./components/AssemblyEditor.vue";
import TraceTable from "./components/TraceTable.vue";

const start = ref(1);
const instructions = ref<Instructions>([]);
const steps = ref<Step[]>([]);

function execute() {
  if (start.value)
    steps.value = Assembler.from_memory(
      instructions.value.map(
        ({ address, label, opcode, operand }) =>
          new Location(address as number, label, opcode, operand),
      ),
    ).execute(start.value);
}
</script>

<template>
  <main>
    <form @submit.prevent="execute">
      <AssemblyEditor v-model="instructions" v-model:start="start" />
      <button type="submit">Execute</button>
    </form>
    <TraceTable :steps="steps" />
  </main>
</template>
