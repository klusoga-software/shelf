import { Link } from "@cloudscape-design/components";

const CountWidget = ({ count }: { count: number }) => {
  return (
    <Link href="repo" variant="awsui-value-large">
      {count}
    </Link>
  );
};

export default CountWidget;
