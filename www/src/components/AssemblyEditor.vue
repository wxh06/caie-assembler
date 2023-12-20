<script setup lang="ts">
import { computed, reactive, watch } from "vue";
import AssemblyEditorInstruction, {
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

const props = defineProps<{ modelValue: Instructions }>();
const emit = defineEmits<{
  (e: "update:modelValue", instructions: Instructions): void;
}>();

const instructions = reactive<Instruction[]>([
  ...props.modelValue,
  emptyInstruction(),
]);
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

watch(instructions, (v) =>
  emit(
    "update:modelValue",
    v
      .map((instruction, i) => ({
        ...instruction,
        address: instructionAddresses.value[i],
      }))
      .slice(0, v.length - 1),
  ),
);
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
