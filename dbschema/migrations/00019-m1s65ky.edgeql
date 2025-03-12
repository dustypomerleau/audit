CREATE MIGRATION m1s65kyojmfh24lwena4z7nzkwdormvd6e4dvuamj6czunfi5dnwmq
    ONTO m1sq2gwagl7svce4nr3lrlqh7aftyqyc6wa3agwvvgo5mojp6v2kfq
{
  ALTER TYPE default::Surgeon {
      ALTER LINK identity {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
