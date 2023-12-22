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
const highlight = ref<number>(0);

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
  <main class="container">
    <div class="row align-items-start">
      <form class="mb-4 col-lg" @submit.prevent="execute">
        <AssemblyEditor
          v-model="instructions"
          v-model:start="start"
          :highlight="highlight"
        />
        <button class="btn btn-primary float-end" type="submit">Execute</button>
      </form>
      <div class="mt-1 col-lg">
        <TraceTable :steps="steps" v-model:highlight="highlight" />
      </div>
    </div>
  </main>
</template>
