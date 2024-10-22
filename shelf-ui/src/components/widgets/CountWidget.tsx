import { Link } from "@cloudscape-design/components";
import { useNavigate } from "react-router-dom";

const CountWidget = ({ count, link }: { count: number; link: string }) => {
  const navigate = useNavigate();

  return (
    <Link
      onClick={() => {
        navigate(link);
      }}
      variant="awsui-value-large"
    >
      {count}
    </Link>
  );
};

export default CountWidget;
