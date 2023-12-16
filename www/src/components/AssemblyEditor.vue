<script setup lang="ts">
import { computed, reactive } from "vue";
import AssemblyEditorInstruction, {
  type Instruction,
} from "./AssemblyEditorInstruction.vue";

const emptyInstruction = (): Instruction => ({
  address: "",
  label: "",
  opcode: "",
  operand: "",
});

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
</script>
<template>
  <AssemblyEditorInstruction
    v-for="(_, i) in instructions"
    v-model="instructions[i]"
    :address="instructionAddresses[i]"
    :address-min="(instructionAddresses[i - 1] ?? 0) + 1"
    :flagged="flagged.has(i)"
    @update:model-value="() => insertLast(i)"
    :key="i"
  />
</template>
