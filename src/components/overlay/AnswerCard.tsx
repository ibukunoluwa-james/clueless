interface AnswerCardProps {
  answer: string;
}

export function AnswerCard({
  answer,
}: AnswerCardProps) {
  return (
    <div className="rounded-xl bg-white/5 p-4">
      <p className="text-sm leading-6 text-zinc-100">
        {answer}
      </p>
    </div>
  );
}