<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import AssemblyEditorInstruction, {
  notEmpty,
  type Instruction,
} from "./AssemblyEditorInstruction.vue";

export type Instructions = {
  address: number;
  label: string;
  opcode: string;
  operand: string;
}[];

const emptyInstruction = (): Instruction => ({
  address: "",
  label: "",
  opcode: "",
  operand: "",
});

defineProps<{ highlight: number }>();
const emit = defineEmits<{
  (e: "submit", instructions: Instructions, start: number): void;
}>();

const start = ref<number>(1);
const instructions = reactive<Instruction[]>([emptyInstruction()]);
const flagged = reactive(new Set());
const instructionAddresses = computed(() => {
  let addresses: number[];
  flagged.clear();
  instructions.forEach((instruction, i) => {
    if (!i) addresses = [instruction.address || 1];
    else {
      addresses.push(
        typeof instruction.address === "number"
          ? instruction.address
          : addresses[i - 1] + 1,
      );
      if (addresses[i] <= addresses[i - 1]) flagged.add(i);
    }
  });
  return addresses!;
});

function insertLast(i: number) {
  if (i === instructions.length - 1) instructions.push(emptyInstruction());
}

const submit = () =>
  emit(
    "submit",
    instructions.filter(notEmpty).map((instruction, i) => ({
      ...instruction,
      address: instructionAddresses.value[i],
    })),
    start.value,
  );
</script>

<template>
  <form @submit.prevent="submit">
    <table class="table text-center">
      <thead>
        <tr>
          <th>Start</th>
          <th>Address</th>
          <th>Label</th>
          <th>Opcode</th>
          <th>Operand</th>
        </tr>
      </thead>
      <tbody>
        <AssemblyEditorInstruction
          v-for="(_, i) in instructions"
          v-model="instructions[i]"
          v-model:start="start"
          :class="{ 'table-active': highlight === instructionAddresses[i] }"
          :address="instructionAddresses[i]"
          :address-min="(instructionAddresses[i - 1] ?? 0) + 1"
          :flagged="flagged.has(i)"
          @focus="insertLast(i)"
          :key="i"
        />
      </tbody>
    </table>
    <button class="btn btn-primary float-end" type="submit">Execute</button>
  </form>
</template>
