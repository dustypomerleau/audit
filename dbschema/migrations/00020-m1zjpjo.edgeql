CREATE MIGRATION m1zjpjodsxzbt2ufyxukpsbvc6lw2wiognwpkpgjzc7vcxni3hmf7a
    ONTO m1s65kyojmfh24lwena4z7nzkwdormvd6e4dvuamj6czunfi5dnwmq
{
  ALTER TYPE default::Surgeon {
      ALTER LINK identity {
          DROP CONSTRAINT std::exclusive;
      };
  };
};
